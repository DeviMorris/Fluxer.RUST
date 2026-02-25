use super::messages::MessageResponse;
use super::webhooks::{WebhookCreateRequest, WebhookResponse};
use crate::enums::{ChannelType, PermissionOverwriteType};
use crate::error::Result;
use crate::flags::Permissions;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::tri::Patch;
use crate::union::PartialUser;
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

    pub async fn get_invites(&self, channel_id: Snowflake) -> Result<Vec<InviteResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/invites").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), Vec<InviteResponse>>(&ep, None)
            .await
    }

    pub async fn create_invite(
        &self,
        channel_id: Snowflake,
        body: &ChannelInviteCreateRequest,
    ) -> Result<InviteResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/invites").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<ChannelInviteCreateRequest, InviteResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_permission_overwrite(
        &self,
        channel_id: Snowflake,
        overwrite_id: Snowflake,
        body: &PermissionOverwriteCreateRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Put,
            "/channels/{channel.id}/permissions/{overwrite.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("overwrite.id", &overwrite_id.to_string()),
            ],
        )?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn delete_permission_overwrite(
        &self,
        channel_id: Snowflake,
        overwrite_id: Snowflake,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/permissions/{overwrite.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("overwrite.id", &overwrite_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_rtc_regions(&self, channel_id: Snowflake) -> Result<Vec<RtcRegionResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/rtc-regions").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), Vec<RtcRegionResponse>>(&ep, None)
            .await
    }

    pub async fn indicate_typing(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/typing").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn add_group_dm_recipient(
        &self,
        channel_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Put,
            "/channels/{channel.id}/recipients/{user.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("channel.id", &channel_id.to_string()),
                ("user.id", &user_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn remove_group_dm_recipient(
        &self,
        channel_id: Snowflake,
        user_id: Snowflake,
        silent: Option<bool>,
    ) -> Result<()> {
        let mut q = QueryValues::new();
        q.insert_opt("silent", silent);
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/channels/{channel.id}/recipients/{user.id}",
        )
        .compile(
            &q,
            &[
                ("channel.id", &channel_id.to_string()),
                ("user.id", &user_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_call_eligibility(
        &self,
        channel_id: Snowflake,
    ) -> Result<CallEligibilityResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/call").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), CallEligibilityResponse>(&ep, None)
            .await
    }

    pub async fn update_call_region(
        &self,
        channel_id: Snowflake,
        body: &CallUpdateRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Patch, "/channels/{channel.id}/call").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_unit::<CallUpdateRequest>(&ep, Some(body))
            .await
    }

    pub async fn end_call(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/call/end").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn ring_call(
        &self,
        channel_id: Snowflake,
        body: &CallRingRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/call/ring").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_unit::<CallRingRequest>(&ep, Some(body))
            .await
    }

    pub async fn stop_ringing_call(
        &self,
        channel_id: Snowflake,
        body: &CallRingRequest,
    ) -> Result<()> {
        let ep =
            Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/call/stop-ringing").compile(
                &QueryValues::new(),
                &[("channel.id", &channel_id.to_string())],
            )?;
        self.http
            .request_unit::<CallRingRequest>(&ep, Some(body))
            .await
    }

    pub async fn list_channel_webhooks(
        &self,
        channel_id: Snowflake,
    ) -> Result<Vec<WebhookResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/channels/{channel.id}/webhooks").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<(), Vec<WebhookResponse>>(&ep, None)
            .await
    }

    pub async fn create_channel_webhook(
        &self,
        channel_id: Snowflake,
        body: &WebhookCreateRequest,
    ) -> Result<WebhookResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/channels/{channel.id}/webhooks").compile(
            &QueryValues::new(),
            &[("channel.id", &channel_id.to_string())],
        )?;
        self.http
            .request_json::<WebhookCreateRequest, WebhookResponse>(&ep, Some(body))
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelInviteCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteResponse {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uses: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter: Option<PartialUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionOverwriteCreateRequest {
    #[serde(rename = "type")]
    pub kind: PermissionOverwriteType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny: Option<Permissions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtcRegionResponse {
    pub id: String,
    pub name: String,
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEligibilityResponse {
    pub ringable: bool,
    pub silent: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallRingRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<Snowflake>>,
}
