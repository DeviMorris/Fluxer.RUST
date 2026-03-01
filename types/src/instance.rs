use serde::{Deserialize, Serialize};

/// Response from `GET /instance`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInstance {
    pub api_code_version: String,
    pub endpoints: InstanceEndpoints,
    #[serde(default)]
    pub captcha: Option<serde_json::Value>,
    #[serde(default)]
    pub features: Option<InstanceFeatures>,
    #[serde(default)]
    pub push: Option<serde_json::Value>,
}

/// Instance API/gateway endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEndpoints {
    pub api: String,
    pub gateway: String,
}

/// Instance feature flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceFeatures {
    #[serde(default)]
    pub voice_enabled: Option<bool>,
}
