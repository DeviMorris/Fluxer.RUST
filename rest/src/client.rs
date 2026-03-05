use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::{FieldError, FluxerApiError, HttpError, RateLimitError, RestError};
use crate::rate_limit::RateLimitManager;

const DEFAULT_API_URL: &str = "https://api.fluxer.app/v1";
const DEFAULT_USER_AGENT: &str = "FluxerBot (Rust, 0.1)";
const DEFAULT_TIMEOUT_SECS: u64 = 15;
const MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone)]
pub struct RestOptions {
    pub api_url: String,
    pub user_agent: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl Default for RestOptions {
    fn default() -> Self {
        Self {
            api_url: DEFAULT_API_URL.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: MAX_RETRIES,
        }
    }
}

#[derive(Clone)]
pub struct Rest {
    http: reqwest::Client,
    options: RestOptions,
    token: Arc<tokio::sync::RwLock<Option<String>>>,
    rate_limiter: Arc<RateLimitManager>,
}

impl Rest {
    pub fn new(options: RestOptions) -> Self {
        let http = reqwest::Client::builder()
            .timeout(options.timeout)
            .build()
            .expect("TLS backend available");
        Self {
            http,
            options,
            token: Arc::new(tokio::sync::RwLock::new(None)),
            rate_limiter: Arc::new(RateLimitManager::new()),
        }
    }

    pub async fn set_token(&self, token: impl Into<String>) {
        let raw = token.into();
        let normalized = if raw.starts_with("Bot ") || raw.starts_with("Bearer ") {
            raw
        } else {
            format!("Bot {raw}")
        };
        let mut guard = self.token.write().await;
        *guard = Some(normalized);
    }

    pub async fn get<T: DeserializeOwned>(&self, route: &str) -> Result<T, RestError> {
        self.request(reqwest::Method::GET, route, Option::<&()>::None)
            .await
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        route: &str,
        body: Option<&(impl Serialize + Sync)>,
    ) -> Result<T, RestError> {
        self.request(reqwest::Method::POST, route, body).await
    }

    pub async fn patch<T: DeserializeOwned>(
        &self,
        route: &str,
        body: Option<&(impl Serialize + Sync)>,
    ) -> Result<T, RestError> {
        self.request(reqwest::Method::PATCH, route, body).await
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        route: &str,
        body: Option<&(impl Serialize + Sync)>,
    ) -> Result<T, RestError> {
        self.request(reqwest::Method::PUT, route, body).await
    }

    pub async fn delete_route(&self, route: &str) -> Result<(), RestError> {
        self.request_empty(reqwest::Method::DELETE, route).await
    }

    pub async fn put_empty(&self, route: &str) -> Result<(), RestError> {
        self.request_empty(reqwest::Method::PUT, route).await
    }

    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        route: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, RestError> {
        self.request_multipart(reqwest::Method::POST, route, form)
            .await
    }

    pub async fn patch_multipart<T: DeserializeOwned>(
        &self,
        route: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, RestError> {
        self.request_multipart(reqwest::Method::PATCH, route, form)
            .await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        route: &str,
        body: Option<&(impl Serialize + Sync)>,
    ) -> Result<T, RestError> {
        let url = format!("{}{}", self.options.api_url, route);
        let mut attempt = 0u32;

        loop {
            self.rate_limiter.wait_if_needed(route).await;

            let mut req = self.http.request(method.clone(), &url);
            req = req.headers(self.build_headers().await);

            if let Some(b) = body {
                req = req.json(b);
            }

            let res = req.send().await?;
            let status = res.status().as_u16();
            self.read_rate_limit_headers_from(route, res.headers());
            let text = res.text().await.unwrap_or_default();

            if status == 429
                && let Ok(rl) = serde_json::from_str::<fluxer_types::RateLimitErrorBody>(&text)
            {
                let global = rl.global.unwrap_or(false);
                if global {
                    self.rate_limiter.set_global(rl.retry_after);
                }
                attempt += 1;
                if attempt < self.options.max_retries {
                    tokio::time::sleep(Duration::from_secs_f64(rl.retry_after)).await;
                    continue;
                }
                return Err(RateLimitError {
                    retry_after: rl.retry_after,
                    global,
                    message: rl.message,
                }
                .into());
            }

            if status >= 400 {
                return Err(self.parse_error(status, &text));
            }

            if text.is_empty() {
                return serde_json::from_str("null").map_err(Into::into);
            }
            return serde_json::from_str(&text).map_err(Into::into);
        }
    }

    async fn request_empty(&self, method: reqwest::Method, route: &str) -> Result<(), RestError> {
        let url = format!("{}{}", self.options.api_url, route);
        self.rate_limiter.wait_if_needed(route).await;

        let req = self
            .http
            .request(method, &url)
            .headers(self.build_headers().await);
        let res = req.send().await?;
        let status = res.status().as_u16();
        self.read_rate_limit_headers_from(route, res.headers());
        let text = res.text().await.unwrap_or_default();

        if status == 429
            && let Ok(rl) = serde_json::from_str::<fluxer_types::RateLimitErrorBody>(&text)
        {
            return Err(RateLimitError {
                retry_after: rl.retry_after,
                global: rl.global.unwrap_or(false),
                message: rl.message,
            }
            .into());
        }

        if status >= 400 {
            return Err(self.parse_error(status, &text));
        }

        Ok(())
    }

    async fn request_multipart<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        route: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, RestError> {
        let url = format!("{}{}", self.options.api_url, route);
        self.rate_limiter.wait_if_needed(route).await;

        let mut headers = self.build_headers().await;
        headers.remove(CONTENT_TYPE);

        let res = self
            .http
            .request(method, &url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        let status = res.status().as_u16();
        self.read_rate_limit_headers_from(route, res.headers());
        let text = res.text().await.unwrap_or_default();

        if status >= 400 {
            return Err(self.parse_error(status, &text));
        }

        serde_json::from_str(&text).map_err(Into::into)
    }

    fn parse_error(&self, status: u16, text: &str) -> RestError {
        if let Ok(api_err) = serde_json::from_str::<fluxer_types::ApiErrorBody>(text) {
            let field_errors: Vec<FieldError> = api_err
                .errors
                .unwrap_or_default()
                .into_iter()
                .map(|e| FieldError {
                    path: e.path,
                    message: e.message,
                })
                .collect();
            if !field_errors.is_empty() {
                let detail = field_errors
                    .iter()
                    .map(|f| format!("  .{}: {}", f.path, f.message))
                    .collect::<Vec<_>>()
                    .join("\n");
                tracing::error!(
                    "API {status} {code}: {msg}\n{detail}",
                    status = status,
                    code = api_err.code,
                    msg = api_err.message,
                    detail = detail,
                );
            }
            FluxerApiError {
                code: api_err.code,
                message: api_err.message,
                status_code: status,
                errors: field_errors,
            }
            .into()
        } else {
            HttpError {
                status_code: status,
                body: text.to_string(),
            }
            .into()
        }
    }

    async fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&self.options.user_agent).expect("valid user agent"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let token = self.token.read().await;
        if let Some(ref t) = *token
            && let Ok(val) = HeaderValue::from_str(t)
        {
            headers.insert(AUTHORIZATION, val);
        }
        headers
    }

    fn read_rate_limit_headers_from(&self, route: &str, headers: &HeaderMap) {
        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u32>().ok());
        let reset_after = headers
            .get("x-ratelimit-reset-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<f64>().ok());
        let is_global = headers
            .get("x-ratelimit-global")
            .and_then(|v| v.to_str().ok())
            .map(|v| v == "true")
            .unwrap_or(false);

        self.rate_limiter
            .update(route, remaining, reset_after, is_global);
    }
}

impl Default for Rest {
    fn default() -> Self {
        Self::new(RestOptions::default())
    }
}
