use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct BucketState {
    remaining: u32,
    reset_at: Instant,
}

/// Per-route and global rate limit tracker.
///
/// Thread-safe via internal `Mutex`. Buckets are keyed by the route
/// path with snowflake IDs replaced by `:id`.
pub struct RateLimitManager {
    buckets: Mutex<HashMap<String, BucketState>>,
    global_reset: Mutex<Option<Instant>>,
}

impl RateLimitManager {
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            global_reset: Mutex::new(None),
        }
    }

    /// Wait if a rate limit is active for the given route.
    pub async fn wait_if_needed(&self, route: &str) {
        if let Some(wait) = self.global_wait() {
            tokio::time::sleep(wait).await;
        }
        if let Some(wait) = self.bucket_wait(route) {
            tokio::time::sleep(wait).await;
        }
    }

    /// Update rate limit state from response headers.
    pub fn update(
        &self,
        route: &str,
        remaining: Option<u32>,
        reset_after_secs: Option<f64>,
        is_global: bool,
    ) {
        let now = Instant::now();

        if is_global {
            if let Some(secs) = reset_after_secs {
                let mut global = self.global_reset.lock().expect("lock not poisoned");
                *global = Some(now + Duration::from_secs_f64(secs));
            }
            return;
        }

        if let (Some(rem), Some(secs)) = (remaining, reset_after_secs) {
            let key = Self::bucket_key(route);
            let mut buckets = self.buckets.lock().expect("lock not poisoned");
            buckets.insert(
                key,
                BucketState {
                    remaining: rem,
                    reset_at: now + Duration::from_secs_f64(secs),
                },
            );
        }
    }

    /// Record a global rate limit hit.
    pub fn set_global(&self, retry_after_secs: f64) {
        let mut global = self.global_reset.lock().expect("lock not poisoned");
        *global = Some(Instant::now() + Duration::from_secs_f64(retry_after_secs));
    }

    fn global_wait(&self) -> Option<Duration> {
        let global = self.global_reset.lock().expect("lock not poisoned");
        global
            .as_ref()
            .and_then(|reset| reset.checked_duration_since(Instant::now()))
    }

    fn bucket_wait(&self, route: &str) -> Option<Duration> {
        let key = Self::bucket_key(route);
        let buckets = self.buckets.lock().expect("lock not poisoned");
        buckets.get(&key).and_then(|state| {
            if state.remaining == 0 {
                state.reset_at.checked_duration_since(Instant::now())
            } else {
                None
            }
        })
    }

    fn bucket_key(route: &str) -> String {
        let mut key = String::with_capacity(route.len());
        let mut prev_was_slash = false;
        for part in route.split('/') {
            if !key.is_empty() || prev_was_slash {
                key.push('/');
            }
            prev_was_slash = true;
            if part.chars().all(|c| c.is_ascii_digit()) && !part.is_empty() {
                key.push_str(":id");
            } else {
                key.push_str(part);
            }
        }
        key
    }
}

impl Default for RateLimitManager {
    fn default() -> Self {
        Self::new()
    }
}
