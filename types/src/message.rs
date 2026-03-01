use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::embed::ApiEmbed;
use crate::user::{ApiGuildMember, ApiUser};
use crate::Snowflake;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    Reply = 19,
}

/// Reaction emoji.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiReactionEmoji {
    pub id: Option<Snowflake>,
    pub name: String,
    #[serde(default)]
    pub animated: Option<bool>,
}

/// Reaction on a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessageReaction {
    pub emoji: ApiReactionEmoji,
    pub count: u32,
    #[serde(default)]
    pub me: Option<bool>,
}

/// Reply/forward reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessageReference {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde(default, rename = "type")]
    pub kind: Option<u8>,
}

/// Call metadata for call-type messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessageCall {
    pub participants: Vec<String>,
    #[serde(default)]
    pub ended_timestamp: Option<String>,
}

/// Snapshot of a forwarded message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessageSnapshot {
    #[serde(default)]
    pub content: Option<String>,
    pub timestamp: String,
    #[serde(default)]
    pub edited_timestamp: Option<String>,
    #[serde(default)]
    pub mentions: Option<Vec<String>>,
    #[serde(default)]
    pub mention_roles: Option<Vec<Snowflake>>,
    #[serde(default)]
    pub embeds: Option<Vec<ApiEmbed>>,
    #[serde(default)]
    pub attachments: Option<Vec<ApiMessageAttachment>>,
    #[serde(default)]
    pub stickers: Option<Vec<ApiMessageSticker>>,
    #[serde(default, rename = "type")]
    pub kind: Option<u8>,
}

/// Message attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessageAttachment {
    pub id: Snowflake,
    pub filename: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub content_hash: Option<String>,
    pub size: u64,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub flags: Option<u32>,
    #[serde(default)]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub waveform: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub expired: Option<bool>,
}

/// Message sticker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessageSticker {
    pub id: Snowflake,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub animated: Option<bool>,
}

/// Message from the API or gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub author: ApiUser,
    #[serde(default)]
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub kind: MessageType,
    #[serde(default)]
    pub flags: Option<u32>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub pinned: bool,
    #[serde(default)]
    pub mention_everyone: Option<bool>,
    #[serde(default)]
    pub tts: Option<bool>,
    #[serde(default)]
    pub mentions: Option<Vec<ApiUser>>,
    #[serde(default)]
    pub mention_roles: Option<Vec<Snowflake>>,
    #[serde(default)]
    pub embeds: Option<Vec<ApiEmbed>>,
    #[serde(default)]
    pub attachments: Option<Vec<ApiMessageAttachment>>,
    #[serde(default)]
    pub stickers: Option<Vec<ApiMessageSticker>>,
    #[serde(default)]
    pub reactions: Option<Vec<ApiMessageReaction>>,
    #[serde(default)]
    pub message_reference: Option<ApiMessageReference>,
    #[serde(default)]
    pub message_snapshots: Option<Vec<ApiMessageSnapshot>>,
    #[serde(default)]
    pub nonce: Option<String>,
    #[serde(default)]
    pub call: Option<ApiMessageCall>,
    #[serde(default)]
    pub referenced_message: Option<Box<ApiMessage>>,
    #[serde(default)]
    pub member: Option<ApiGuildMember>,
}
