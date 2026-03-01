use std::fmt;

/// API error returned by the Fluxer backend.
#[derive(Debug)]
pub struct FluxerApiError {
    pub code: String,
    pub message: String,
    pub status_code: u16,
    pub errors: Vec<FieldError>,
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub path: String,
    pub message: String,
}

impl fmt::Display for FluxerApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.status_code, self.code, self.message)
    }
}

impl std::error::Error for FluxerApiError {}

/// Generic HTTP transport error.
#[derive(Debug)]
pub struct HttpError {
    pub status_code: u16,
    pub body: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}: {}", self.status_code, self.body)
    }
}

impl std::error::Error for HttpError {}

/// Rate limit error.
#[derive(Debug)]
pub struct RateLimitError {
    pub retry_after: f64,
    pub global: bool,
    pub message: String,
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rate limited: retry after {}s (global={})",
            self.retry_after, self.global
        )
    }
}

impl std::error::Error for RateLimitError {}

/// Unified error type for the REST crate.
#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("{0}")]
    Api(#[from] FluxerApiError),
    #[error("{0}")]
    Http(#[from] HttpError),
    #[error("{0}")]
    RateLimit(#[from] RateLimitError),
    #[error("request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
