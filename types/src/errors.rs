use serde::{Deserialize, Serialize};

/// API error body from Fluxer responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub errors: Option<Vec<ApiFieldError>>,
}

/// Individual field error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiFieldError {
    pub path: String,
    pub message: String,
    #[serde(default)]
    pub code: Option<String>,
}

/// Rate limit error body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitErrorBody {
    pub code: String,
    pub message: String,
    pub retry_after: f64,
    #[serde(default)]
    pub global: Option<bool>,
}
