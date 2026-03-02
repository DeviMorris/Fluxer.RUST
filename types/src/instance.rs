use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEndpoints {
    pub api: String,
    pub gateway: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceFeatures {
    #[serde(default)]
    pub voice_enabled: Option<bool>,
}
