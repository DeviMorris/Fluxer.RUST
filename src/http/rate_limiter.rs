use crate::error::{RateLimitError, Result, StateError};
use crate::http::CompiledEndpoint;
use reqwest::Response;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify, OwnedSemaphorePermit, Semaphore};
use tokio::time::{Duration, Instant, sleep};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimiterState>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(RateLimiterState::default())),
        }
    }

    pub async fn acquire(
        &self,
        endpoint: &CompiledEndpoint,
        shutdown: &Notify,
    ) -> Result<RateLimitLease> {
        let route_key = route_key(endpoint);
        loop {
            let (wait_until, semaphore, bucket_key) = {
                let mut state = self.inner.lock().await;
                let global_until = state.global_until;
                let bucket_key = state
                    .route_to_bucket
                    .get(&route_key)
                    .cloned()
                    .unwrap_or_else(|| route_key.clone());

                let bucket = state
                    .buckets
                    .entry(bucket_key.clone())
                    .or_insert_with(BucketState::new);

                let now = Instant::now();
                let mut wait_until = now;

                if let Some(global_until) = global_until {
                    if global_until > wait_until {
                        wait_until = global_until;
                    }
                }

                if bucket.remaining == 0 && bucket.reset_at > wait_until {
                    wait_until = bucket.reset_at;
                }

                (wait_until, bucket.semaphore.clone(), bucket_key)
            };

            if wait_until > Instant::now() {
                let wait = wait_until.saturating_duration_since(Instant::now());
                tokio::select! {
                    _ = sleep(wait) => {}
                    _ = shutdown.notified() => {
                        return Err(StateError::Closed.into());
                    }
                }
            }

            let permit = tokio::select! {
                result = semaphore.acquire_owned() => result,
                _ = shutdown.notified() => {
                    return Err(StateError::Closed.into());
                }
            }
            .map_err(|_| StateError::Closed)?;

            return Ok(RateLimitLease {
                route_key,
                bucket_key,
                _permit: permit,
            });
        }
    }

    pub async fn update(
        &self,
        endpoint: &CompiledEndpoint,
        lease: &RateLimitLease,
        response: Option<&Response>,
    ) -> Result<Option<RateLimitError>> {
        let Some(response) = response else {
            return Ok(None);
        };

        let mut state = self.inner.lock().await;
        let mut bucket_key = lease.bucket_key.clone();
        let status = response.status();
        let headers = response.headers();

        let mut observed_bucket_id = None;
        if let Some(bucket_id) = header_str(headers, "x-ratelimit-bucket") {
            observed_bucket_id = Some(bucket_id.to_owned());
            let target_key = if endpoint.major_params.is_empty() {
                bucket_id.to_owned()
            } else {
                format!("{bucket_id}:{}", endpoint.major_params)
            };

            state
                .route_to_bucket
                .insert(lease.route_key.clone(), target_key.clone());

            if target_key != bucket_key && !state.buckets.contains_key(&target_key) {
                if let Some(old) = state.buckets.remove(&bucket_key) {
                    state.buckets.insert(target_key.clone(), old);
                    bucket_key = target_key;
                }
            } else if target_key != bucket_key {
                bucket_key = target_key;
            }
        }

        let mut set_global_until = None;
        let bucket = state
            .buckets
            .entry(bucket_key)
            .or_insert_with(BucketState::new);
        if let Some(bucket_id) = observed_bucket_id {
            bucket.id = Some(bucket_id);
        }

        if status.as_u16() == 429 {
            let retry_after = retry_after(headers).unwrap_or_else(|| Duration::from_secs(1));
            let global = header_str(headers, "x-ratelimit-global").is_some();
            let until = Instant::now() + retry_after;

            if global {
                set_global_until = Some(until);
            } else {
                bucket.remaining = 0;
                bucket.reset_at = until;
            }

            let rate_limit_err = RateLimitError::new(retry_after, bucket.id.clone(), global);

            if let Some(until) = set_global_until {
                state.global_until = Some(until);
            }
            return Ok(Some(rate_limit_err));
        }

        if let Some(limit) = parse_i32_header(headers, "x-ratelimit-limit") {
            bucket.limit = limit;
        }
        if let Some(remaining) = parse_i32_header(headers, "x-ratelimit-remaining") {
            bucket.remaining = remaining;
        } else if bucket.remaining > 0 {
            bucket.remaining -= 1;
        }
        if let Some(reset_after) = retry_after(headers) {
            bucket.reset_at = Instant::now() + reset_after;
        }

        Ok(None)
    }
}

#[derive(Debug)]
pub struct RateLimitLease {
    route_key: String,
    bucket_key: String,
    _permit: OwnedSemaphorePermit,
}

#[derive(Debug)]
struct BucketState {
    id: Option<String>,
    reset_at: Instant,
    remaining: i32,
    limit: i32,
    semaphore: Arc<Semaphore>,
}

impl BucketState {
    fn new() -> Self {
        Self {
            id: None,
            reset_at: Instant::now(),
            remaining: 1,
            limit: -1,
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }
}

#[derive(Debug, Default)]
struct RateLimiterState {
    global_until: Option<Instant>,
    route_to_bucket: HashMap<String, String>,
    buckets: HashMap<String, BucketState>,
}

fn route_key(endpoint: &CompiledEndpoint) -> String {
    let mut key = format!("{}+{}", endpoint.method.as_str(), endpoint.route);
    if !endpoint.major_params.is_empty() {
        key.push('+');
        key.push_str(&endpoint.major_params);
    }
    key
}

fn parse_i32_header(headers: &reqwest::header::HeaderMap, name: &str) -> Option<i32> {
    header_str(headers, name).and_then(|v| v.parse::<i32>().ok())
}

fn retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    if let Some(v) = header_str(headers, "x-ratelimit-reset-after") {
        if let Ok(sec) = v.parse::<f64>() {
            return Some(Duration::from_secs_f64(sec.max(0.0)));
        }
    }
    if let Some(v) = header_str(headers, "retry-after") {
        if let Ok(sec) = v.parse::<f64>() {
            return Some(Duration::from_secs_f64(sec.max(0.0)));
        }
    }
    None
}

fn header_str<'a>(headers: &'a reqwest::header::HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Endpoint, HttpMethod, QueryValues};

    #[test]
    fn route_key_with_major() {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}")
            .compile(&QueryValues::new(), &[("channel.id", "42")])
            .expect("compile");
        assert_eq!(route_key(&ep), "GET+/channels/{channel.id}+channel.id=42");
    }

    #[test]
    fn retry_after_prefers_reset_after() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-ratelimit-reset-after", "2.5".parse().expect("header"));
        headers.insert("retry-after", "9".parse().expect("header"));
        let got = retry_after(&headers).expect("retry after");
        assert_eq!(got, Duration::from_millis(2500));
    }

    #[test]
    fn retry_after_header() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("retry-after", "3".parse().expect("header"));
        let got = retry_after(&headers).expect("retry after");
        assert_eq!(got, Duration::from_secs(3));
    }
}
