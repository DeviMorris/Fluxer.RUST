use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::Snowflake;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildVerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildMfaLevel {
    None = 0,
    Elevated = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GuildExplicitContentFilter {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    AllMessages = 0,
    OnlyMentions = 1,
}

/// Guild from `GET /guilds/{id}` or gateway `GUILD_CREATE`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGuild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    #[serde(default)]
    pub banner_width: Option<u32>,
    #[serde(default)]
    pub banner_height: Option<u32>,
    #[serde(default)]
    pub splash: Option<String>,
    #[serde(default)]
    pub splash_width: Option<u32>,
    #[serde(default)]
    pub splash_height: Option<u32>,
    #[serde(default)]
    pub splash_card_alignment: Option<u32>,
    #[serde(default)]
    pub embed_splash: Option<String>,
    #[serde(default)]
    pub embed_splash_width: Option<u32>,
    #[serde(default)]
    pub embed_splash_height: Option<u32>,
    #[serde(default)]
    pub vanity_url_code: Option<String>,
    pub owner_id: Snowflake,
    #[serde(default)]
    pub system_channel_id: Option<Snowflake>,
    #[serde(default)]
    pub system_channel_flags: Option<u32>,
    #[serde(default)]
    pub rules_channel_id: Option<Snowflake>,
    #[serde(default)]
    pub afk_channel_id: Option<Snowflake>,
    #[serde(default)]
    pub afk_timeout: Option<u32>,
    #[serde(default)]
    pub features: Vec<String>,
    pub verification_level: GuildVerificationLevel,
    pub mfa_level: GuildMfaLevel,
    #[serde(default)]
    pub nsfw_level: Option<u32>,
    pub explicit_content_filter: GuildExplicitContentFilter,
    pub default_message_notifications: DefaultMessageNotifications,
    #[serde(default)]
    pub disabled_operations: Option<u32>,
    #[serde(default)]
    pub message_history_cutoff: Option<String>,
    #[serde(default)]
    pub permissions: Option<String>,
}

/// Audit log change entry value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogChange {
    pub key: String,
    #[serde(default)]
    pub old_value: Option<serde_json::Value>,
    #[serde(default)]
    pub new_value: Option<serde_json::Value>,
}

/// Audit log entry from `GET /guilds/{id}/audit-logs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGuildAuditLogEntry {
    pub id: String,
    pub action_type: u32,
    #[serde(default)]
    pub user_id: Option<Snowflake>,
    #[serde(default)]
    pub target_id: Option<Snowflake>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub changes: Option<Vec<AuditLogChange>>,
}

/// Audit log user (minimal).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogUser {
    pub id: Snowflake,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub discriminator: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
}

/// Audit log webhook (minimal).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogWebhook {
    pub id: Snowflake,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
}

/// Response from `GET /guilds/{id}/audit-logs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGuildAuditLog {
    pub audit_log_entries: Vec<ApiGuildAuditLogEntry>,
    #[serde(default)]
    pub users: Vec<AuditLogUser>,
    #[serde(default)]
    pub webhooks: Vec<AuditLogWebhook>,
}

/// Response from `GET /guilds/{id}/vanity-url`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVanityUrl {
    pub code: Option<String>,
    pub uses: u32,
}

/// Request body for guild feature toggles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGuildFeatureToggle {
    pub enabled: bool,
}
