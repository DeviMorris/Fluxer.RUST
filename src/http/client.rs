use crate::error::{ApiError, Error, Result, StateError, TransportError};
use crate::http::{API_BASE, AuthPolicy, CompiledEndpoint, RateLimiter};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use tokio::sync::Notify;
use tokio::time::{Duration, sleep};

#[derive(Debug, Clone)]
/// Async HTTP client used by Fluxer services.
///
/// Owns a single `reqwest::Client` instance, rate-limit coordination,
/// retry policy and graceful shutdown state.
pub struct HttpClient {
    inner: reqwest::Client,
    cfg: Arc<HttpClientConfig>,
    limiter: RateLimiter,
    shutdown: Arc<ShutdownState>,
}

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub base_url: String,
    pub bot_token: Option<String>,
    pub user_agent: String,
    pub timeout: Duration,
    pub retry: RetryPolicy,
    pub allow_env_proxy: bool,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            base_url: API_BASE.trim_end_matches('/').to_owned(),
            bot_token: None,
            user_agent: format!("fluxer-rust/{}", env!("CARGO_PKG_VERSION")),
            timeout: Duration::from_secs(20),
            retry: RetryPolicy::default(),
            allow_env_proxy: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 10,
            base_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(10),
        }
    }
}

impl RetryPolicy {
    pub fn backoff(&self, attempt: u32) -> Duration {
        let exp = 1u128 << attempt.saturating_sub(1).min(20);
        let millis = self.base_delay.as_millis().saturating_mul(exp);
        let capped = millis.min(self.max_delay.as_millis());
        Duration::from_millis(capped as u64)
    }
}

impl HttpClient {
    pub fn new(cfg: HttpClientConfig) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&cfg.user_agent)
                .map_err(|e| TransportError::Other(format!("invalid user-agent: {e}")))?,
        );

        let mut builder = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(cfg.timeout);
        if !cfg.allow_env_proxy {
            builder = builder.no_proxy();
        }
        let client = builder.build().map_err(map_reqwest_error)?;

        Ok(Self {
            inner: client,
            cfg: Arc::new(cfg),
            limiter: RateLimiter::new(),
            shutdown: Arc::new(ShutdownState::default()),
        })
    }

    pub fn reqwest(&self) -> &reqwest::Client {
        &self.inner
    }

    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.limiter
    }

    pub async fn shutdown(&self) {
        self.shutdown.closed.store(true, Ordering::SeqCst);
        self.shutdown.notify.notify_waiters();

        loop {
            if self.shutdown.in_flight.load(Ordering::SeqCst) == 0 {
                break;
            }
            self.shutdown.notify.notified().await;
        }
    }

    pub async fn request_json<B, T>(
        &self,
        endpoint: &CompiledEndpoint,
        body: Option<&B>,
    ) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let response = self.request(endpoint, body).await?;
        if response.status().as_u16() == 204 {
            return serde_json::from_value(serde_json::Value::Null)
                .map_err(|e| Error::Protocol(crate::error::ProtocolError::Json(e)));
        }

        let bytes = response.bytes().await.map_err(map_reqwest_error)?;
        serde_json::from_slice(&bytes)
            .map_err(|e| Error::Protocol(crate::error::ProtocolError::Json(e)))
    }

    pub async fn request_unit<B>(&self, endpoint: &CompiledEndpoint, body: Option<&B>) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let _ = self.request(endpoint, body).await?;
        Ok(())
    }

    pub async fn request_json_with_auth<B, T>(
        &self,
        endpoint: &CompiledEndpoint,
        body: Option<&B>,
        auth_header: Option<&str>,
    ) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let response = self.request_with_auth(endpoint, body, auth_header).await?;
        if response.status().as_u16() == 204 {
            return serde_json::from_value(serde_json::Value::Null)
                .map_err(|e| Error::Protocol(crate::error::ProtocolError::Json(e)));
        }

        let bytes = response.bytes().await.map_err(map_reqwest_error)?;
        serde_json::from_slice(&bytes)
            .map_err(|e| Error::Protocol(crate::error::ProtocolError::Json(e)))
    }

    pub async fn request_unit_with_auth<B>(
        &self,
        endpoint: &CompiledEndpoint,
        body: Option<&B>,
        auth_header: Option<&str>,
    ) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let _ = self.request_with_auth(endpoint, body, auth_header).await?;
        Ok(())
    }

    async fn request<B>(
        &self,
        endpoint: &CompiledEndpoint,
        body: Option<&B>,
    ) -> Result<reqwest::Response>
    where
        B: Serialize + ?Sized,
    {
        self.request_with_auth(endpoint, body, None).await
    }

    async fn request_with_auth<B>(
        &self,
        endpoint: &CompiledEndpoint,
        body: Option<&B>,
        auth_header: Option<&str>,
    ) -> Result<reqwest::Response>
    where
        B: Serialize + ?Sized,
    {
        if self.shutdown.closed.load(Ordering::SeqCst) {
            return Err(StateError::Closed.into());
        }

        let _guard = InFlightGuard::new(self.shutdown.clone())?;
        let mut attempt = 0;
        loop {
            attempt += 1;
            let lease = self
                .limiter
                .acquire(endpoint, &self.shutdown.notify)
                .await?;

            let request = self.build_request(endpoint, body, auth_header)?;
            let response = request.send().await;

            match response {
                Ok(response) => {
                    let status = response.status();
                    let rl = self
                        .limiter
                        .update(endpoint, &lease, Some(&response))
                        .await?;

                    if status.as_u16() == 429 {
                        let Some(rate_err) = rl else {
                            return Err(TransportError::Other(
                                "429 without ratelimit payload".to_owned(),
                            )
                            .into());
                        };
                        if attempt > self.cfg.retry.max_retries {
                            return Err(Error::RateLimit(rate_err));
                        }

                        let delay = rate_err.retry_after.max(self.cfg.retry.backoff(attempt));
                        if self.sleep_or_shutdown(delay).await.is_err() {
                            return Err(StateError::Closed.into());
                        }
                        continue;
                    }

                    if status.is_success() {
                        return Ok(response);
                    }

                    return Err(parse_api_error(response).await);
                }
                Err(err) => {
                    let _ = self.limiter.update(endpoint, &lease, None).await?;
                    if !is_retryable_transport(&err) || attempt > self.cfg.retry.max_retries {
                        return Err(map_reqwest_error(err));
                    }
                    let delay = self.cfg.retry.backoff(attempt);
                    if self.sleep_or_shutdown(delay).await.is_err() {
                        return Err(StateError::Closed.into());
                    }
                }
            }
        }
    }

    fn build_request<B>(
        &self,
        endpoint: &CompiledEndpoint,
        body: Option<&B>,
        auth_header: Option<&str>,
    ) -> Result<reqwest::RequestBuilder>
    where
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.cfg.base_url, endpoint.url);
        let method = reqwest::Method::from_bytes(endpoint.method.as_str().as_bytes())
            .map_err(|e| TransportError::Other(format!("invalid method: {e}")))?;

        let mut builder = self.inner.request(method, url);

        if let Some(auth_header) = auth_header {
            builder = builder.header(AUTHORIZATION, auth_header);
        } else if endpoint.auth == AuthPolicy::Bot {
            let token = self
                .cfg
                .bot_token
                .as_deref()
                .ok_or(StateError::Missing("bot_token"))?;
            builder = builder.header(AUTHORIZATION, format!("Bot {token}"));
        }

        if let Some(body) = body {
            builder = builder.header(CONTENT_TYPE, "application/json").json(body);
        }

        Ok(builder)
    }

    async fn sleep_or_shutdown(&self, delay: Duration) -> Result<()> {
        tokio::select! {
            _ = sleep(delay) => Ok(()),
            _ = self.shutdown.notify.notified() => Err(StateError::Closed.into()),
        }
    }
}

#[derive(Debug, Default)]
struct ShutdownState {
    closed: AtomicBool,
    in_flight: AtomicUsize,
    notify: Notify,
}

#[derive(Debug)]
struct InFlightGuard {
    state: Arc<ShutdownState>,
}

impl InFlightGuard {
    fn new(state: Arc<ShutdownState>) -> Result<Self> {
        if state.closed.load(Ordering::SeqCst) {
            return Err(StateError::Closed.into());
        }
        state.in_flight.fetch_add(1, Ordering::SeqCst);
        Ok(Self { state })
    }
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        if self.state.in_flight.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.state.notify.notify_waiters();
        }
    }
}

fn is_retryable_transport(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}

fn map_reqwest_error(err: reqwest::Error) -> Error {
    if err.is_timeout() {
        Error::Transport(TransportError::Timeout)
    } else if err.is_request() {
        Error::Transport(TransportError::Canceled)
    } else {
        Error::Transport(TransportError::Other(err.to_string()))
    }
}

async fn parse_api_error(response: reqwest::Response) -> Error {
    let status = response.status().as_u16();
    let bytes = response.bytes().await.unwrap_or_default();
    parse_api_error_parts(status, &bytes)
}

fn parse_api_error_parts(status: u16, bytes: &[u8]) -> Error {
    #[derive(serde::Deserialize)]
    struct ApiBody {
        code: Option<i64>,
        message: Option<String>,
    }
    let payload: Option<ApiBody> = serde_json::from_slice(bytes).ok();
    let message = payload
        .as_ref()
        .and_then(|v| v.message.clone())
        .unwrap_or_else(|| String::from_utf8_lossy(bytes).to_string());
    let code = payload.and_then(|v| v.code);
    Error::Api(ApiError::new(status, code, message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_growth() {
        let p = RetryPolicy::default();
        assert!(p.backoff(2) > p.backoff(1));
    }

    #[test]
    fn backoff_capped() {
        let p = RetryPolicy {
            max_retries: 10,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(3),
        };
        assert_eq!(p.backoff(5), Duration::from_secs(3));
    }

    #[test]
    fn api_parse_json() {
        let err = parse_api_error_parts(400, br#"{"code":10008,"message":"Unknown Message"}"#);
        match err {
            Error::Api(v) => {
                assert_eq!(v.status, 400);
                assert_eq!(v.code, Some(10008));
                assert_eq!(v.message, "Unknown Message");
            }
            _ => panic!("wrong error"),
        }
    }

    #[test]
    fn api_parse_text() {
        let err = parse_api_error_parts(502, b"Bad Gateway");
        match err {
            Error::Api(v) => {
                assert_eq!(v.status, 502);
                assert_eq!(v.code, None);
                assert_eq!(v.message, "Bad Gateway");
            }
            _ => panic!("wrong error"),
        }
    }
}
