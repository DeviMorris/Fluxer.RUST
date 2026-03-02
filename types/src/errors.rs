use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub errors: Option<Vec<ApiFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiFieldError {
    pub path: String,
    pub message: String,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitErrorBody {
    pub code: String,
    pub message: String,
    pub retry_after: f64,
    #[serde(default)]
    pub global: Option<bool>,
}
