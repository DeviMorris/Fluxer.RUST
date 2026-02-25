use super::channels::ChannelResponse;
use crate::error::Result;
use crate::flags::UserFlags;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct UsersApi {
    http: HttpClient,
}

impl UsersApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_me(&self) -> Result<UserPrivateResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), UserPrivateResponse>(&ep, None)
            .await
    }

    pub async fn update_me(&self, body: &UserProfileUpdateRequest) -> Result<UserPrivateResponse> {
        let ep =
            Endpoint::new(HttpMethod::Patch, "/users/@me").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<UserProfileUpdateRequest, UserPrivateResponse>(&ep, Some(body))
            .await
    }

    pub async fn get_my_channels(&self) -> Result<Vec<ChannelResponse>> {
        let ep =
            Endpoint::new(HttpMethod::Get, "/users/@me/channels").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<ChannelResponse>>(&ep, None)
            .await
    }

    pub async fn create_dm(&self, body: &CreatePrivateChannelRequest) -> Result<ChannelResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/channels")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<CreatePrivateChannelRequest, ChannelResponse>(&ep, Some(body))
            .await
    }

    pub async fn pin_channel(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Put, "/users/@me/channels/{channel_id}/pin").compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn unpin_channel(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/channels/{channel_id}/pin")
            .compile(&QueryValues::new(), &[("channel_id", &channel_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_connections(&self) -> Result<Vec<ConnectionResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/connections")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<ConnectionResponse>>(&ep, None)
            .await
    }

    pub async fn create_connection(
        &self,
        body: &CreateConnectionRequest,
    ) -> Result<ConnectionVerificationResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/connections")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<CreateConnectionRequest, ConnectionVerificationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn authorize_bluesky(
        &self,
        body: &BlueskyAuthorizeRequest,
    ) -> Result<BlueskyAuthorizeResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/connections/bluesky/authorize")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<BlueskyAuthorizeRequest, BlueskyAuthorizeResponse>(&ep, Some(body))
            .await
    }

    pub async fn reorder_connections(&self, body: &ReorderConnectionsRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/connections/reorder")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn verify_connection(
        &self,
        body: &VerifyAndCreateConnectionRequest,
    ) -> Result<ConnectionResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/connections/verify")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<VerifyAndCreateConnectionRequest, ConnectionResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_connection(
        &self,
        connection_type: &str,
        connection_id: &str,
        body: &UpdateConnectionRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/users/@me/connections/{type}/{connection_id}",
        )
        .compile(
            &QueryValues::new(),
            &[("type", connection_type), ("connection_id", connection_id)],
        )?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn delete_connection(
        &self,
        connection_type: &str,
        connection_id: &str,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/users/@me/connections/{type}/{connection_id}",
        )
        .compile(
            &QueryValues::new(),
            &[("type", connection_type), ("connection_id", connection_id)],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn verify_connection_by_id(
        &self,
        connection_type: &str,
        connection_id: &str,
    ) -> Result<ConnectionResponse> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/connections/{type}/{connection_id}/verify",
        )
        .compile(
            &QueryValues::new(),
            &[("type", connection_type), ("connection_id", connection_id)],
        )?;
        self.http
            .request_json::<(), ConnectionResponse>(&ep, None)
            .await
    }

    pub async fn list_mentions(
        &self,
        query: &ListMentionsQuery,
    ) -> Result<Vec<Value>> {
        let mut q = QueryValues::new();
        q.insert_opt("limit", query.limit);
        q.insert_opt("roles", query.roles);
        q.insert_opt("everyone", query.everyone);
        q.insert_opt("guilds", query.guilds);
        q.insert_opt("before", query.before);

        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/mentions").compile(&q, &[])?;
        self.http.request_json::<(), Vec<Value>>(&ep, None).await
    }

    pub async fn delete_mention(&self, message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/mentions/{message_id}")
            .compile(&QueryValues::new(), &[("message_id", &message_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_relationships(&self) -> Result<Vec<RelationshipResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/relationships")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<RelationshipResponse>>(&ep, None)
            .await
    }

    pub async fn create_relationship_by_tag(
        &self,
        body: &FriendRequestByTagRequest,
    ) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/relationships")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<FriendRequestByTagRequest, RelationshipResponse>(&ep, Some(body))
            .await
    }

    pub async fn create_relationship(&self, user_id: Snowflake) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http
            .request_json::<(), RelationshipResponse>(&ep, None)
            .await
    }

    pub async fn put_relationship(
        &self,
        user_id: Snowflake,
        body: &RelationshipTypePutRequest,
    ) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Put, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http
            .request_json::<RelationshipTypePutRequest, RelationshipResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_relationship(&self, user_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn update_relationship_nickname(
        &self,
        user_id: Snowflake,
        body: &RelationshipNicknameUpdateRequest,
    ) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http
            .request_json::<RelationshipNicknameUpdateRequest, RelationshipResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn get_settings(&self) -> Result<UserSettingsResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/settings")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), UserSettingsResponse>(&ep, None)
            .await
    }

    pub async fn update_settings(
        &self,
        body: &UserSettingsUpdateRequest,
    ) -> Result<UserSettingsResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/settings")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<UserSettingsUpdateRequest, UserSettingsResponse>(&ep, Some(body))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivateResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_bounced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfileUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_masked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_timestamp_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_sequence_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_enabled_override: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_dismissed_premium_onboarding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_unread_gift_inventory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_mobile_client: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrivateChannelRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<Snowflake>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub name: String,
    pub verified: bool,
    pub visibility_flags: i32,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionVerificationResponse {
    pub token: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub id: String,
    pub instructions: String,
    pub initiation_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "type")]
    pub connection_type: String,
    pub identifier: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueskyAuthorizeRequest {
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueskyAuthorizeResponse {
    pub authorize_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderConnectionsRequest {
    pub connection_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAndCreateConnectionRequest {
    pub initiation_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConnectionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMentionsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guilds: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipResponse {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: i32,
    pub user: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendRequestByTagRequest {
    pub username: String,
    pub discriminator: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationshipTypePutRequest {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNicknameUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsUpdateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
