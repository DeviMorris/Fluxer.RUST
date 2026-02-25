use crate::error::Result;
use crate::flags::MessageFlags;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::tri::Patch;
use crate::union::PartialUser;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct MessagesApi {
    http: HttpClient,
}

impl MessagesApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn send_message(
        &self,
        channel_id: Snowflake,
        body: &SendMessageRequest,
    ) -> Result<MessageResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/messages").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<SendMessageRequest, MessageResponse>(&ep, Some(body))
            .await
    }

    pub async fn get_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
    ) -> Result<MessageResponse> {
        let ep = Endpoint::new(
            HttpMethod::Get,
            "/channels/{channel.id}/messages/{message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<(), MessageResponse>(&ep, None)
            .await
    }

    pub async fn edit_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        body: &EditMessageRequest,
    ) -> Result<MessageResponse> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/channels/{channel.id}/messages/{message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<EditMessageRequest, MessageResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/messages/{message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn bulk_delete_messages(
        &self,
        channel_id: Snowflake,
        body: &BulkDeleteMessagesRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/channels/{channel.id}/messages/bulk-delete",
        )
        .compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_unit::<BulkDeleteMessagesRequest>(&ep, Some(body))
            .await
    }

    pub async fn add_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Put,
            "/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
                ("emoji", emoji),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn remove_own_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
                ("emoji", emoji),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn remove_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
        target_id: Snowflake,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/messages/{message.id}/reactions/{emoji}/{target.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
                ("emoji", emoji),
                ("target.id", &target_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn clear_reactions(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/messages/{message.id}/reactions",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn clear_emoji_reactions(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/messages/{message.id}/reactions/{emoji}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
                ("emoji", emoji),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_reactions(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<ReactionUsersListResponse> {
        let ep = Endpoint::new(
            HttpMethod::Get,
            "/channels/{channel.id}/messages/{message.id}/reactions/{emoji}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
                ("emoji", emoji),
            ],
        )?;
        self.http
            .request_json::<(), ReactionUsersListResponse>(&ep, None)
            .await
    }

    pub async fn get_pins(&self, channel_id: Snowflake) -> Result<ChannelPinsResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/messages/pins").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), ChannelPinsResponse>(&ep, None)
            .await
    }

    pub async fn pin_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Put, "/channels/{channel.id}/pins/{message.id}")
            .compile(
                &QueryValues::new(),
                &[
                    ("channel.id", &channel_id.to_string()),
                    ("message.id", &message_id.to_string()),
                ],
            )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn unpin_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/pins/{message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn ack_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        body: &MessageAckRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/channels/{channel.id}/messages/{message.id}/ack",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
            ],
        )?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn delete_ack(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/channels/{channel.id}/messages/ack").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn delete_attachment(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        attachment_id: Snowflake,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/messages/{message.id}/attachments/{attachment.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("message.id", &message_id.to_string()),
                ("attachment.id", &attachment_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn ack_pins(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/pins/ack").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn schedule_message(
        &self,
        channel_id: Snowflake,
        body: &ScheduledMessageCreateRequest,
    ) -> Result<ScheduledMessageResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/messages/schedule")
            .compile(
                &QueryValues::new(),
                &[("channel.id", &channel_id.to_string())],
            )?;
        self.http
            .request_json::<ScheduledMessageCreateRequest, ScheduledMessageResponse>(
                &ep,
                Some(body),
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageRequest {
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub content: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub flags: Patch<MessageFlags>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub embeds: Patch<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub attachments: Patch<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub allowed_mentions: Patch<Value>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub message_reference: Patch<Value>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub components: Patch<Vec<Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAckRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDeleteMessagesRequest {
    pub messages: Vec<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<PartialUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Value>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMessageCreateRequest {
    pub scheduled_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_local_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<Snowflake>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorite_meme_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMessageResponse {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub scheduled_at: String,
    pub scheduled_local_at: String,
    pub timezone: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    pub payload: ScheduledMessagePayload,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalidated_at: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMessagePayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<Snowflake>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorite_meme_id: Option<Snowflake>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionUsersListResponse {
    #[serde(default)]
    pub users: Vec<PartialUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPinsResponse {
    #[serde(default)]
    pub items: Vec<ChannelPinResponse>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPinResponse {
    pub message: MessageResponse,
    pub pinned_at: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
