use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::user::ApiUser;
use crate::Snowflake;

/// Channel type enum aligned with Fluxer API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildLink = 5,
    GuildLinkExtended = 998,
}

/// Permission overwrite type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

/// Permission overwrite on a channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiChannelOverwrite {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub kind: OverwriteType,
    pub allow: String,
    pub deny: String,
}

/// Minimal channel (id, type required).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiChannelPartial {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: u16,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub parent_id: Option<Snowflake>,
}

/// Full channel from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiChannel {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub kind: u16,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub topic: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub owner_id: Option<Snowflake>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub parent_id: Option<Snowflake>,
    #[serde(default)]
    pub bitrate: Option<u32>,
    #[serde(default)]
    pub user_limit: Option<u32>,
    #[serde(default)]
    pub rtc_region: Option<String>,
    #[serde(default)]
    pub last_message_id: Option<Snowflake>,
    #[serde(default)]
    pub last_pin_timestamp: Option<String>,
    #[serde(default)]
    pub permission_overwrites: Option<Vec<ApiChannelOverwrite>>,
    #[serde(default)]
    pub recipients: Option<Vec<ApiUser>>,
    #[serde(default)]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub rate_limit_per_user: Option<u32>,
}
