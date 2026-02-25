use super::messages::MessageResponse;
use crate::enums::ChannelType;
use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::tri::Patch;
use crate::union::PermissionOverwrite;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct ChannelsApi {
    http: HttpClient,
}

impl ChannelsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_channel(&self, channel_id: Snowflake) -> Result<ChannelResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), ChannelResponse>(&ep, None)
            .await
    }

    pub async fn create_channel(
        &self,
        guild_id: Snowflake,
        body: &CreateChannelRequest,
    ) -> Result<ChannelResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/channels")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<CreateChannelRequest, ChannelResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_channel(
        &self,
        channel_id: Snowflake,
        body: &UpdateChannelRequest,
    ) -> Result<ChannelResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/channels/{channel.id}").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<UpdateChannelRequest, ChannelResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_channel(&self, channel_id: Snowflake) -> Result<ChannelResponse> {
        let ep = Endpoint::new(HttpMethod::Delete, "/channels/{channel.id}").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), ChannelResponse>(&ep, None)
            .await
    }

    pub async fn list_channel_messages(
        &self,
        channel_id: Snowflake,
        query: &ListChannelMessagesQuery,
    ) -> Result<Vec<MessageResponse>> {
        let mut q = QueryValues::new();
        q.insert_opt("limit", query.limit);
        q.insert_opt("before", query.before);
        q.insert_opt("after", query.after);
        q.insert_opt("around", query.around);

        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/messages")
            .compile(&q, &[("channel.id", &channel_id.to_string())])?;
        self.http
            .request_json::<(), Vec<MessageResponse>>(&ep, None)
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChannelMessagesQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub around: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelRequest {
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub name: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub topic: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub bitrate: Patch<u32>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub user_limit: Patch<u16>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub parent_id: Patch<Snowflake>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub nsfw: Patch<bool>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub position: Patch<i32>,
    #[serde(default, skip_serializing_if = "Patch::is_omitted")]
    pub permission_overwrites: Patch<Vec<PermissionOverwrite>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelResponse {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
