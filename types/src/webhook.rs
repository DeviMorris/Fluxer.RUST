use serde::{Deserialize, Serialize};

use crate::user::ApiUser;
use crate::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiWebhook {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: String,
    pub avatar: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    pub user: ApiUser,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookTokenUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}
