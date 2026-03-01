use serde::{Deserialize, Serialize};

use crate::channel::ApiChannelPartial;
use crate::user::ApiUser;
use crate::Snowflake;

/// Partial guild in invite context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGuildPartial {
    pub id: Snowflake,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub splash: Option<String>,
    #[serde(default)]
    pub features: Option<Vec<String>>,
}

/// Invite from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInvite {
    pub code: String,
    #[serde(rename = "type")]
    pub kind: u32,
    pub guild: ApiGuildPartial,
    pub channel: ApiChannelPartial,
    #[serde(default)]
    pub inviter: Option<ApiUser>,
    #[serde(default)]
    pub member_count: Option<u32>,
    #[serde(default)]
    pub presence_count: Option<u32>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub temporary: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub uses: Option<u32>,
    #[serde(default)]
    pub max_uses: Option<u32>,
    #[serde(default)]
    pub max_age: Option<u32>,
}
