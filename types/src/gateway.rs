use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::channel::ApiChannel;
use crate::emoji::ApiEmoji;
use crate::guild::ApiGuild;
use crate::role::ApiRole;
use crate::sticker::ApiSticker;
use crate::user::{ApiGuildMember, ApiUser};
use crate::Snowflake;

// ── Opcodes ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GatewayOpcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}

// ── Outgoing payloads ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayIdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayIdentifyData {
    pub token: String,
    pub intents: u64,
    pub properties: GatewayIdentifyProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<(u32, u32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<GatewayPresenceUpdateSendData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResumeData {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayCustomStatus {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub emoji_name: Option<String>,
    #[serde(default)]
    pub emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayActivity {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: u8,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPresenceUpdateSendData {
    #[serde(default)]
    pub since: Option<u64>,
    #[serde(default)]
    pub activities: Option<Vec<GatewayActivity>>,
    #[serde(default)]
    pub custom_status: Option<GatewayCustomStatus>,
    pub status: String,
    #[serde(default)]
    pub afk: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayVoiceStateUpdateSendData {
    pub guild_id: Snowflake,
    pub channel_id: Option<Snowflake>,
    #[serde(default)]
    pub self_mute: Option<bool>,
    #[serde(default)]
    pub self_deaf: Option<bool>,
    #[serde(default)]
    pub self_video: Option<bool>,
    #[serde(default)]
    pub self_stream: Option<bool>,
    #[serde(default)]
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRequestGuildMembersData {
    pub guild_id: Snowflake,
    #[serde(default)]
    pub query: Option<String>,
    pub limit: u32,
}

// ── Incoming payloads ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayHelloData {
    pub heartbeat_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayApplication {
    pub id: Snowflake,
    pub flags: u64,
}

/// `READY` event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReadyData {
    pub v: u32,
    pub user: ApiUser,
    pub guilds: Vec<GatewayReadyGuild>,
    pub session_id: String,
    #[serde(default)]
    pub shard: Option<(u32, u32)>,
    pub application: GatewayApplication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReadyGuild {
    #[serde(flatten)]
    pub guild: ApiGuild,
    #[serde(default)]
    pub unavailable: Option<bool>,
}

/// `MESSAGE_DELETE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMessageDeleteData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub author_id: Option<Snowflake>,
}

/// `MESSAGE_DELETE_BULK` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMessageDeleteBulkData {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// Emoji in reaction events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReactionEmoji {
    #[serde(default)]
    pub id: Option<Snowflake>,
    pub name: String,
    #[serde(default)]
    pub animated: Option<bool>,
}

/// `MESSAGE_REACTION_ADD` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReactionAddData {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub emoji: GatewayReactionEmoji,
}

/// `MESSAGE_REACTION_REMOVE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReactionRemoveData {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub emoji: GatewayReactionEmoji,
}

/// `MESSAGE_REACTION_REMOVE_EMOJI` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReactionRemoveEmojiData {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub emoji: GatewayReactionEmoji,
}

/// `MESSAGE_REACTION_REMOVE_ALL` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReactionRemoveAllData {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// `GUILD_DELETE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildDeleteData {
    pub id: Snowflake,
    #[serde(default)]
    pub unavailable: Option<bool>,
}

/// `CHANNEL_UPDATE_BULK` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayChannelUpdateBulkData {
    pub channels: Vec<ApiChannel>,
}

/// `CHANNEL_RECIPIENT_ADD` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayChannelRecipientData {
    pub channel_id: Snowflake,
    pub user: ApiUser,
}

/// `GUILD_MEMBER_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildMemberUpdateData {
    pub guild_id: Snowflake,
    pub roles: Vec<Snowflake>,
    pub user: ApiUser,
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub joined_at: Option<String>,
    #[serde(default)]
    pub premium_since: Option<String>,
    #[serde(default)]
    pub communication_disabled_until: Option<String>,
}

/// `GUILD_MEMBER_REMOVE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildMemberRemoveData {
    pub guild_id: Snowflake,
    pub user: ApiUser,
}

/// `GUILD_MEMBERS_CHUNK` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildMembersChunkData {
    pub guild_id: Snowflake,
    pub members: Vec<ApiGuildMember>,
    pub chunk_index: u32,
    pub chunk_count: u32,
    #[serde(default)]
    pub nonce: Option<String>,
}

/// `GUILD_BAN_ADD` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildBanAddData {
    pub guild_id: Snowflake,
    pub user: ApiUser,
    #[serde(default)]
    pub reason: Option<String>,
}

/// `GUILD_BAN_REMOVE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildBanRemoveData {
    pub guild_id: Snowflake,
    pub user: ApiUser,
}

/// `INVITE_DELETE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayInviteDeleteData {
    pub code: String,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// `TYPING_START` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayTypingStartData {
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub timestamp: u64,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
}

/// `GUILD_ROLE_CREATE` / `GUILD_ROLE_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildRoleData {
    pub guild_id: Snowflake,
    pub role: ApiRole,
}

/// `GUILD_ROLE_DELETE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildRoleDeleteData {
    pub guild_id: Snowflake,
    pub role_id: Snowflake,
}

/// `GUILD_ROLE_UPDATE_BULK` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildRoleUpdateBulkData {
    pub guild_id: Snowflake,
    pub roles: Vec<ApiRole>,
}

/// `VOICE_STATE_UPDATE` (incoming) payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayVoiceStateUpdateData {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    #[serde(default)]
    pub member: Option<ApiGuildMember>,
    pub session_id: String,
    #[serde(default)]
    pub connection_id: Option<String>,
    #[serde(default)]
    pub deaf: Option<bool>,
    #[serde(default)]
    pub mute: Option<bool>,
    #[serde(default)]
    pub self_deaf: Option<bool>,
    #[serde(default)]
    pub self_mute: Option<bool>,
    #[serde(default)]
    pub self_video: Option<bool>,
    #[serde(default)]
    pub self_stream: Option<bool>,
    #[serde(default)]
    pub suppress: Option<bool>,
}

/// `VOICE_SERVER_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayVoiceServerUpdateData {
    pub token: String,
    pub guild_id: Snowflake,
    pub endpoint: Option<String>,
    #[serde(default)]
    pub connection_id: Option<String>,
}

/// `GUILD_EMOJIS_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildEmojisUpdateData {
    pub guild_id: Snowflake,
    pub emojis: Vec<ApiEmoji>,
}

/// `GUILD_STICKERS_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildStickersUpdateData {
    pub guild_id: Snowflake,
    pub stickers: Vec<ApiSticker>,
}

/// `GUILD_INTEGRATIONS_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildIntegrationsUpdateData {
    pub guild_id: Snowflake,
}

/// `CHANNEL_PINS_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayChannelPinsUpdateData {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub last_pin_timestamp: Option<String>,
}

/// `PRESENCE_UPDATE` (incoming) payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPresenceUpdateData {
    pub user: PresenceUser,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub activities: Option<Vec<GatewayActivity>>,
    #[serde(default)]
    pub custom_status: Option<GatewayCustomStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUser {
    pub id: Snowflake,
}

/// `WEBHOOKS_UPDATE` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayWebhooksUpdateData {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}

/// `GUILD_SCHEDULED_EVENT` payload (create/update/delete).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayGuildScheduledEventData {
    pub guild_id: Snowflake,
    pub id: Snowflake,
}

/// Response from `GET /gateway/bot`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayBotResponse {
    pub url: String,
    pub shards: u32,
    pub session_start_limit: SessionStartLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u64,
    pub max_concurrency: u32,
}

/// Raw incoming gateway payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReceivePayload {
    pub op: GatewayOpcode,
    #[serde(default)]
    pub d: Option<serde_json::Value>,
    #[serde(default)]
    pub s: Option<u64>,
    #[serde(default)]
    pub t: Option<String>,
}
