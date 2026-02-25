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
