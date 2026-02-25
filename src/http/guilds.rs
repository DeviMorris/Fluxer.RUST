use super::channels::{ChannelResponse, InviteResponse};
use super::roles::RoleResponse;
use super::webhooks::WebhookResponse;
use crate::error::Result;
use crate::flags::SystemChannelFlags;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use crate::union::PartialUser;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct GuildsApi {
    http: HttpClient,
}

impl GuildsApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_guild(&self, guild_id: Snowflake) -> Result<GuildResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http.request_json::<(), GuildResponse>(&ep, None).await
    }

    pub async fn update_guild(
        &self,
        guild_id: Snowflake,
        body: &GuildUpdateRequest,
    ) -> Result<GuildResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildUpdateRequest, GuildResponse>(&ep, Some(body))
            .await
    }

    pub async fn create_guild(&self, body: &GuildCreateRequest) -> Result<GuildResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<GuildCreateRequest, GuildResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_guild(&self, guild_id: Snowflake, body: &GuildDeleteRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/delete")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_unit::<GuildDeleteRequest>(&ep, Some(body))
            .await
    }

    pub async fn list_guild_channels(&self, guild_id: Snowflake) -> Result<Vec<ChannelResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/channels")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<ChannelResponse>>(&ep, None)
            .await
    }

    pub async fn get_guild_roles(&self, guild_id: Snowflake) -> Result<Vec<RoleResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/roles")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<RoleResponse>>(&ep, None)
            .await
    }

    pub async fn update_channel_positions(
        &self,
        guild_id: Snowflake,
        body: &ChannelPositionUpdateRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/channels")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_unit::<ChannelPositionUpdateRequest>(&ep, Some(body))
            .await
    }

    pub async fn update_role_positions(
        &self,
        guild_id: Snowflake,
        body: &GuildRolePositionsRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/roles")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_unit::<GuildRolePositionsRequest>(&ep, Some(body))
            .await
    }

    pub async fn list_bans(&self, guild_id: Snowflake) -> Result<Vec<GuildBanResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/bans")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<GuildBanResponse>>(&ep, None)
            .await
    }

    pub async fn get_audit_log(
        &self,
        guild_id: Snowflake,
        query: &GuildAuditLogQuery,
    ) -> Result<GuildAuditLogListResponse> {
        let mut q = QueryValues::new();
        q.insert_opt("limit", query.limit);
        q.insert_opt("before", query.before);
        q.insert_opt("after", query.after);
        q.insert_opt("user_id", query.user_id);
        q.insert_opt("action_type", query.action_type);

        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/audit-logs")
            .compile(&q, &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), GuildAuditLogListResponse>(&ep, None)
            .await
    }

    pub async fn get_guild_invites(&self, guild_id: Snowflake) -> Result<Vec<InviteResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/invites")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<InviteResponse>>(&ep, None)
            .await
    }

    pub async fn get_guild_vanity_url(
        &self,
        guild_id: Snowflake,
    ) -> Result<GuildVanityUrlResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/vanity-url")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), GuildVanityUrlResponse>(&ep, None)
            .await
    }

    pub async fn update_guild_vanity_url(
        &self,
        guild_id: Snowflake,
        body: &GuildVanityUrlUpdateRequest,
    ) -> Result<GuildVanityUrlUpdateResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/vanity-url")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildVanityUrlUpdateRequest, GuildVanityUrlUpdateResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn list_guild_webhooks(&self, guild_id: Snowflake) -> Result<Vec<WebhookResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/webhooks")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<WebhookResponse>>(&ep, None)
            .await
    }

    pub async fn transfer_guild_ownership(
        &self,
        guild_id: Snowflake,
        body: &GuildTransferOwnershipRequest,
    ) -> Result<GuildResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/transfer-ownership")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildTransferOwnershipRequest, GuildResponse>(&ep, Some(body))
            .await
    }

    pub async fn toggle_detached_banner(
        &self,
        guild_id: Snowflake,
        body: &EnabledToggleRequest,
    ) -> Result<GuildResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/detached-banner")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<EnabledToggleRequest, GuildResponse>(&ep, Some(body))
            .await
    }

    pub async fn toggle_text_channel_flexible_names(
        &self,
        guild_id: Snowflake,
        body: &EnabledToggleRequest,
    ) -> Result<GuildResponse> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/guilds/{guild.id}/text-channel-flexible-names",
        )
        .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<EnabledToggleRequest, GuildResponse>(&ep, Some(body))
            .await
    }

    pub async fn get_guild_discovery_status(
        &self,
        guild_id: Snowflake,
    ) -> Result<DiscoveryStatusResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/discovery")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), DiscoveryStatusResponse>(&ep, None)
            .await
    }

    pub async fn apply_for_guild_discovery(
        &self,
        guild_id: Snowflake,
        body: &DiscoveryApplicationRequest,
    ) -> Result<DiscoveryApplicationResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/discovery")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<DiscoveryApplicationRequest, DiscoveryApplicationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn update_guild_discovery_application(
        &self,
        guild_id: Snowflake,
        body: &DiscoveryApplicationPatchRequest,
    ) -> Result<DiscoveryApplicationResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/discovery")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<DiscoveryApplicationPatchRequest, DiscoveryApplicationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn delete_guild_discovery_application(&self, guild_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/guilds/{guild.id}/discovery")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_emojis(&self, guild_id: Snowflake) -> Result<Vec<GuildEmojiResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/emojis")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<GuildEmojiResponse>>(&ep, None)
            .await
    }

    pub async fn create_emoji(
        &self,
        guild_id: Snowflake,
        body: &GuildEmojiCreateRequest,
    ) -> Result<GuildEmojiResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/emojis")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildEmojiCreateRequest, GuildEmojiResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_emoji(
        &self,
        guild_id: Snowflake,
        emoji_id: Snowflake,
        body: &GuildEmojiUpdateRequest,
    ) -> Result<GuildEmojiResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/guilds/{guild.id}/emojis/{emoji.id}").compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("emoji.id", &emoji_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<GuildEmojiUpdateRequest, GuildEmojiResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_emoji(&self, guild_id: Snowflake, emoji_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/guilds/{guild.id}/emojis/{emoji.id}")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild.id", &guild_id.to_string()),
                    ("emoji.id", &emoji_id.to_string()),
                ],
            )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn bulk_create_emojis(
        &self,
        guild_id: Snowflake,
        body: &GuildEmojiBulkCreateRequest,
    ) -> Result<GuildEmojiBulkCreateResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/emojis/bulk")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildEmojiBulkCreateRequest, GuildEmojiBulkCreateResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn list_stickers(&self, guild_id: Snowflake) -> Result<Vec<GuildStickerResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/stickers")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<(), Vec<GuildStickerResponse>>(&ep, None)
            .await
    }

    pub async fn create_sticker(
        &self,
        guild_id: Snowflake,
        body: &GuildStickerCreateRequest,
    ) -> Result<GuildStickerResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/stickers")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildStickerCreateRequest, GuildStickerResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_sticker(
        &self,
        guild_id: Snowflake,
        sticker_id: Snowflake,
        body: &GuildStickerUpdateRequest,
    ) -> Result<GuildStickerResponse> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/guilds/{guild.id}/stickers/{sticker.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("sticker.id", &sticker_id.to_string()),
            ],
        )?;
        self.http
            .request_json::<GuildStickerUpdateRequest, GuildStickerResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_sticker(&self, guild_id: Snowflake, sticker_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/guilds/{guild.id}/stickers/{sticker.id}",
        )
        .compile(
            &QueryValues::new(),
            &[
                ("guild.id", &guild_id.to_string()),
                ("sticker.id", &sticker_id.to_string()),
            ],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn bulk_create_stickers(
        &self,
        guild_id: Snowflake,
        body: &GuildStickerBulkCreateRequest,
    ) -> Result<GuildStickerBulkCreateResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/guilds/{guild.id}/stickers/bulk")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<GuildStickerBulkCreateRequest, GuildStickerBulkCreateResponse>(
                &ep,
                Some(body),
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildResponse {
    pub id: Snowflake,
    pub name: String,
    pub owner_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuildUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_splash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash_card_alignment: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_history_cutoff: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildCreateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_features: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuildDeleteRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTransferOwnershipRequest {
    pub new_owner_id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledToggleRequest {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveryApplicationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveryApplicationPatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryApplicationResponse {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStatusResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<DiscoveryApplicationResponse>,
    pub eligible: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildVanityUrlResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub uses: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuildVanityUrlUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildVanityUrlUpdateResponse {
    pub code: String,
}

pub type ChannelPositionUpdateRequest = Vec<ChannelPositionUpdateItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPositionUpdateItem {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<Snowflake>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preceding_sibling_id: Option<Option<Snowflake>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_permissions: Option<bool>,
}

pub type GuildRolePositionsRequest = Vec<GuildRolePositionItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRolePositionItem {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildBanResponse {
    pub user: PartialUser,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub moderator_id: Snowflake,
    pub banned_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuildAuditLogQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildAuditLogListResponse {
    #[serde(default)]
    pub audit_log_entries: Vec<GuildAuditLogEntryResponse>,
    #[serde(default)]
    pub users: Vec<PartialUser>,
    #[serde(default)]
    pub webhooks: Vec<AuditLogWebhookResponse>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildAuditLogEntryResponse {
    pub id: Snowflake,
    pub action_type: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<Value>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogWebhookResponse {
    pub id: Snowflake,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEmojiResponse {
    pub id: Snowflake,
    pub name: String,
    pub animated: bool,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEmojiCreateRequest {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEmojiUpdateRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEmojiBulkCreateRequest {
    pub emojis: Vec<GuildEmojiCreateRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEmojiBulkCreateResponse {
    pub success: Vec<GuildEmojiResponse>,
    pub failed: Vec<GuildBulkFailedItem>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildBulkFailedItem {
    pub name: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildStickerResponse {
    pub id: Snowflake,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub animated: bool,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildStickerCreateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildStickerUpdateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildStickerBulkCreateRequest {
    pub stickers: Vec<GuildStickerCreateRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildStickerBulkCreateResponse {
    pub success: Vec<GuildStickerResponse>,
    pub failed: Vec<GuildBulkFailedItem>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
