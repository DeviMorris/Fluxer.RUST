use std::collections::HashMap;

use fluxer_types::channel::ApiChannelOverwrite;
use fluxer_types::guild::ApiGuild;
use fluxer_types::Snowflake;

use crate::structures::role::Role;
use crate::util::cdn::{self, CdnOptions};

/// A guild (server) on Fluxer.
///
/// Contains server metadata, settings, and cached role mappings.
#[derive(Debug, Clone)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub splash: Option<String>,
    pub owner_id: Snowflake,
    pub features: Vec<String>,
    pub verification_level: u8,
    pub mfa_level: u8,
    pub nsfw_level: Option<u32>,
    pub explicit_content_filter: u8,
    pub default_message_notifications: u8,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: Option<u32>,
    pub rules_channel_id: Option<Snowflake>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<u32>,
    pub vanity_url_code: Option<String>,
    pub permissions: Option<String>,
    pub roles: HashMap<Snowflake, Role>,
}

impl Guild {
    pub fn from_api(data: &ApiGuild) -> Self {
        Self {
            id: data.id.clone(),
            name: data.name.clone(),
            icon: data.icon.clone(),
            banner: data.banner.clone(),
            splash: data.splash.clone(),
            owner_id: data.owner_id.clone(),
            features: data.features.clone(),
            verification_level: data.verification_level as u8,
            mfa_level: data.mfa_level as u8,
            nsfw_level: data.nsfw_level,
            explicit_content_filter: data.explicit_content_filter as u8,
            default_message_notifications: data.default_message_notifications as u8,
            system_channel_id: data.system_channel_id.clone(),
            system_channel_flags: data.system_channel_flags,
            rules_channel_id: data.rules_channel_id.clone(),
            afk_channel_id: data.afk_channel_id.clone(),
            afk_timeout: data.afk_timeout,
            vanity_url_code: data.vanity_url_code.clone(),
            permissions: data.permissions.clone(),
            roles: HashMap::new(),
        }
    }

    /// Get the guild icon URL.
    pub fn icon_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_guild_icon_url(&self.id, self.icon.as_deref(), opts)
    }

    /// Get the guild banner URL.
    pub fn banner_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_banner_url(&self.id, self.banner.as_deref(), opts)
    }

    /// Get the guild splash URL.
    pub fn splash_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_guild_splash_url(&self.id, self.splash.as_deref(), opts)
    }

    /// Fetch all roles for this guild.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] on network failure.
    pub async fn fetch_roles(
        &mut self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<Role>> {
        let data: Vec<fluxer_types::role::ApiRole> =
            rest.get(&fluxer_types::Routes::guild_roles(&self.id)).await?;
        let roles: Vec<Role> = data
            .iter()
            .map(|r| Role::from_api(r, &self.id))
            .collect();
        for role in &roles {
            self.roles.insert(role.id.clone(), role.clone());
        }
        Ok(roles)
    }

    /// Create a role in this guild.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn create_role(
        &mut self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_types::role::CreateRoleBody,
    ) -> crate::Result<Role> {
        let data: fluxer_types::role::ApiRole = rest
            .post(&fluxer_types::Routes::guild_roles(&self.id), Some(body))
            .await?;
        let role = Role::from_api(&data, &self.id);
        self.roles.insert(role.id.clone(), role.clone());
        Ok(role)
    }

    /// Add a role to a member.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn add_role_to_member(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
        role_id: &str,
    ) -> crate::Result<()> {
        rest.put_empty(&fluxer_types::Routes::guild_member_role(
            &self.id, user_id, role_id,
        ))
        .await?;
        Ok(())
    }

    /// Remove a role from a member.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn remove_role_from_member(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
        role_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_member_role(
            &self.id, user_id, role_id,
        ))
        .await?;
        Ok(())
    }

    /// Ban a member from this guild.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `BAN_MEMBERS`.
    pub async fn ban(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
        reason: Option<&str>,
    ) -> crate::Result<()> {
        let body = reason.map(|r| serde_json::json!({ "reason": r }));
        let _: serde_json::Value = rest
            .put(
                &fluxer_types::Routes::guild_ban(&self.id, user_id),
                body.as_ref(),
            )
            .await?;
        Ok(())
    }

    /// Unban a user from this guild.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `BAN_MEMBERS`.
    pub async fn unban(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_ban(&self.id, user_id))
            .await?;
        Ok(())
    }

    /// Kick a member from this guild.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `KICK_MEMBERS`.
    pub async fn kick(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_member(&self.id, user_id))
            .await?;
        Ok(())
    }

    /// Fetch bans for this guild.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `BAN_MEMBERS`.
    pub async fn fetch_bans(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::ban::ApiBan>> {
        let bans: Vec<fluxer_types::ban::ApiBan> =
            rest.get(&fluxer_types::Routes::guild_bans(&self.id)).await?;
        Ok(bans)
    }

    /// Fetch channels for this guild.
    pub async fn fetch_channels(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::channel::ApiChannel>> {
        let channels: Vec<fluxer_types::channel::ApiChannel> =
            rest.get(&fluxer_types::Routes::guild_channels(&self.id)).await?;
        Ok(channels)
    }

    /// Fetch invites for this guild.
    pub async fn fetch_invites(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::invite::ApiInvite>> {
        let invites: Vec<fluxer_types::invite::ApiInvite> =
            rest.get(&fluxer_types::Routes::guild_invites(&self.id)).await?;
        Ok(invites)
    }

    /// Fetch webhooks for this guild.
    pub async fn fetch_webhooks(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::webhook::ApiWebhook>> {
        let webhooks: Vec<fluxer_types::webhook::ApiWebhook> =
            rest.get(&fluxer_types::Routes::guild_webhooks(&self.id)).await?;
        Ok(webhooks)
    }

    /// Fetch audit logs for this guild.
    pub async fn fetch_audit_logs(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<fluxer_types::guild::ApiGuildAuditLog> {
        let logs: fluxer_types::guild::ApiGuildAuditLog =
            rest.get(&fluxer_types::Routes::guild_audit_logs(&self.id)).await?;
        Ok(logs)
    }

    /// Fetch emojis for this guild.
    pub async fn fetch_emojis(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::emoji::ApiEmoji>> {
        let emojis: Vec<fluxer_types::emoji::ApiEmoji> =
            rest.get(&fluxer_types::Routes::guild_emojis(&self.id)).await?;
        Ok(emojis)
    }

    /// Fetch stickers for this guild.
    pub async fn fetch_stickers(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::sticker::ApiSticker>> {
        let stickers: Vec<fluxer_types::sticker::ApiSticker> =
            rest.get(&fluxer_types::Routes::guild_stickers(&self.id)).await?;
        Ok(stickers)
    }

    /// Get the permission overwrites from a channel in this guild (for permission computation).
    ///
    /// Returns an empty vec if the source doesn't have overwrites.
    pub fn channel_overwrites(&self, _channel_id: &str) -> Vec<ApiChannelOverwrite> {
        Vec::new()
    }
}

impl std::fmt::Display for Guild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
