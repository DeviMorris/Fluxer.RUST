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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageRequest {
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub content: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub flags: Patch<MessageFlags>,
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
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
