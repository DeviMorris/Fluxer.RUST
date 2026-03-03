use std::collections::HashMap;

use fluxer_types::Snowflake;
use fluxer_types::channel::ApiChannelOverwrite;
use fluxer_types::guild::ApiGuild;
use serde_json::Value;

use crate::structures::role::Role;
use crate::util::cdn::{self, CdnOptions};

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
    pub channels: Vec<Snowflake>,
    pub emojis: Vec<Snowflake>,
    pub member_count: Option<u64>,
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
            channels: Vec::new(),
            emojis: Vec::new(),
            member_count: None,
        }
    }

    pub fn from_id(id: impl Into<Snowflake>) -> Self {
        Self {
            id: id.into(),
            name: String::new(),
            icon: None,
            banner: None,
            splash: None,
            owner_id: String::new(),
            features: Vec::new(),
            verification_level: 0,
            mfa_level: 0,
            nsfw_level: None,
            explicit_content_filter: 0,
            default_message_notifications: 0,
            system_channel_id: None,
            system_channel_flags: None,
            rules_channel_id: None,
            afk_channel_id: None,
            afk_timeout: None,
            vanity_url_code: None,
            permissions: None,
            roles: HashMap::new(),
            channels: Vec::new(),
            emojis: Vec::new(),
            member_count: None,
        }
    }

    pub fn patch(&mut self, data: &ApiGuild) {
        self.name = data.name.clone();
        self.icon = data.icon.clone();
        self.banner = data.banner.clone();
        self.splash = data.splash.clone();
        self.owner_id = data.owner_id.clone();
        self.features = data.features.clone();
        self.verification_level = data.verification_level as u8;
        self.mfa_level = data.mfa_level as u8;
        self.nsfw_level = data.nsfw_level;
        self.explicit_content_filter = data.explicit_content_filter as u8;
        self.default_message_notifications = data.default_message_notifications as u8;
        self.system_channel_id = data.system_channel_id.clone();
        self.system_channel_flags = data.system_channel_flags;
        self.rules_channel_id = data.rules_channel_id.clone();
        self.afk_channel_id = data.afk_channel_id.clone();
        self.afk_timeout = data.afk_timeout;
        self.vanity_url_code = data.vanity_url_code.clone();
        self.permissions = data.permissions.clone();
    }

    pub fn icon_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_guild_icon_url(&self.id, self.icon.as_deref(), opts)
    }

    pub fn banner_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_banner_url(&self.id, self.banner.as_deref(), opts)
    }

    pub fn splash_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_guild_splash_url(&self.id, self.splash.as_deref(), opts)
    }

    pub async fn edit(&self, rest: &fluxer_rest::Rest, body: &Value) -> crate::Result<ApiGuild> {
        let guild: ApiGuild = rest
            .patch(&fluxer_types::Routes::guild(&self.id), Some(body))
            .await?;
        Ok(guild)
    }

    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_delete(&self.id))
            .await?;
        Ok(())
    }

    pub async fn create_channel(
        &self,
        rest: &fluxer_rest::Rest,
        body: &Value,
    ) -> crate::Result<fluxer_types::channel::ApiChannel> {
        let ch: fluxer_types::channel::ApiChannel = rest
            .post(&fluxer_types::Routes::guild_channels(&self.id), Some(body))
            .await?;
        Ok(ch)
    }

    pub async fn fetch_member(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<fluxer_types::user::ApiGuildMember> {
        let member: fluxer_types::user::ApiGuildMember = rest
            .get(&fluxer_types::Routes::guild_member(&self.id, user_id))
            .await?;
        Ok(member)
    }

    pub async fn fetch_members(
        &self,
        rest: &fluxer_rest::Rest,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> crate::Result<Vec<fluxer_types::user::ApiGuildMember>> {
        let mut route = fluxer_types::Routes::guild_members(&self.id);
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(a) = after {
            params.push(format!("after={a}"));
        }
        if !params.is_empty() {
            route = format!("{route}?{}", params.join("&"));
        }
        let members: Vec<fluxer_types::user::ApiGuildMember> = rest.get(&route).await?;
        Ok(members)
    }

    pub async fn fetch_vanity_url(&self, rest: &fluxer_rest::Rest) -> crate::Result<Value> {
        let data: Value = rest
            .get(&fluxer_types::Routes::guild_vanity_url(&self.id))
            .await?;
        Ok(data)
    }

    pub async fn transfer_ownership(
        &self,
        rest: &fluxer_rest::Rest,
        new_owner_id: &str,
    ) -> crate::Result<ApiGuild> {
        let body = serde_json::json!({ "owner_id": new_owner_id });
        let guild: ApiGuild = rest
            .post(
                &fluxer_types::Routes::guild_transfer_ownership(&self.id),
                Some(&body),
            )
            .await?;
        Ok(guild)
    }

    pub async fn fetch_roles(&mut self, rest: &fluxer_rest::Rest) -> crate::Result<Vec<Role>> {
        let data: Vec<fluxer_types::role::ApiRole> = rest
            .get(&fluxer_types::Routes::guild_roles(&self.id))
            .await?;
        let roles: Vec<Role> = data.iter().map(|r| Role::from_api(r, &self.id)).collect();
        for role in &roles {
            self.roles.insert(role.id.clone(), role.clone());
        }
        Ok(roles)
    }

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

    pub async fn ban(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
        reason: Option<&str>,
    ) -> crate::Result<()> {
        let body = reason.map(|r| serde_json::json!({ "reason": r }));
        let _: Value = rest
            .put(
                &fluxer_types::Routes::guild_ban(&self.id, user_id),
                body.as_ref(),
            )
            .await?;
        Ok(())
    }

    pub async fn unban(&self, rest: &fluxer_rest::Rest, user_id: &str) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_ban(&self.id, user_id))
            .await?;
        Ok(())
    }

    pub async fn kick(&self, rest: &fluxer_rest::Rest, user_id: &str) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_member(&self.id, user_id))
            .await?;
        Ok(())
    }

    pub async fn fetch_bans(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::ban::ApiBan>> {
        let bans: Vec<fluxer_types::ban::ApiBan> = rest
            .get(&fluxer_types::Routes::guild_bans(&self.id))
            .await?;
        Ok(bans)
    }

    pub async fn fetch_channels(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::channel::ApiChannel>> {
        let channels: Vec<fluxer_types::channel::ApiChannel> = rest
            .get(&fluxer_types::Routes::guild_channels(&self.id))
            .await?;
        Ok(channels)
    }

    pub async fn fetch_invites(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::invite::ApiInvite>> {
        let invites: Vec<fluxer_types::invite::ApiInvite> = rest
            .get(&fluxer_types::Routes::guild_invites(&self.id))
            .await?;
        Ok(invites)
    }

    pub async fn fetch_webhooks(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::webhook::ApiWebhook>> {
        let webhooks: Vec<fluxer_types::webhook::ApiWebhook> = rest
            .get(&fluxer_types::Routes::guild_webhooks(&self.id))
            .await?;
        Ok(webhooks)
    }

    pub async fn fetch_audit_logs(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<fluxer_types::guild::ApiGuildAuditLog> {
        let logs: fluxer_types::guild::ApiGuildAuditLog = rest
            .get(&fluxer_types::Routes::guild_audit_logs(&self.id))
            .await?;
        Ok(logs)
    }

    pub async fn fetch_emojis(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::emoji::ApiEmoji>> {
        let emojis: Vec<fluxer_types::emoji::ApiEmoji> = rest
            .get(&fluxer_types::Routes::guild_emojis(&self.id))
            .await?;
        Ok(emojis)
    }

    pub async fn fetch_emoji(
        &self,
        rest: &fluxer_rest::Rest,
        emoji_id: &str,
    ) -> crate::Result<fluxer_types::emoji::ApiEmoji> {
        let emoji: fluxer_types::emoji::ApiEmoji = rest
            .get(&fluxer_types::Routes::guild_emoji(&self.id, emoji_id))
            .await?;
        Ok(emoji)
    }

    pub async fn fetch_stickers(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::sticker::ApiSticker>> {
        let stickers: Vec<fluxer_types::sticker::ApiSticker> = rest
            .get(&fluxer_types::Routes::guild_stickers(&self.id))
            .await?;
        Ok(stickers)
    }

    pub async fn set_role_positions(
        &self,
        rest: &fluxer_rest::Rest,
        positions: &Value,
    ) -> crate::Result<Vec<fluxer_types::role::ApiRole>> {
        let raw: Value = rest
            .patch(
                &fluxer_types::Routes::guild_roles(&self.id),
                Some(positions),
            )
            .await?;
        let roles: Vec<fluxer_types::role::ApiRole> = match raw {
            Value::Null => vec![],
            other => {
                serde_json::from_value(other).map_err(|e| crate::Error::Other(e.to_string()))?
            }
        };
        Ok(roles)
    }

    pub async fn set_channel_positions(
        &self,
        rest: &fluxer_rest::Rest,
        positions: &Value,
    ) -> crate::Result<()> {
        let _: Value = rest
            .patch(
                &fluxer_types::Routes::guild_channels(&self.id),
                Some(positions),
            )
            .await?;
        Ok(())
    }

    pub async fn create_emoji(
        &self,
        rest: &fluxer_rest::Rest,
        name: &str,
        image: &str,
        role_ids: Option<&[String]>,
    ) -> crate::Result<fluxer_types::emoji::ApiEmoji> {
        let mut body = serde_json::json!({ "name": name, "image": image });
        if let Some(roles) = role_ids {
            body["roles"] = serde_json::Value::from(roles.to_vec());
        }
        let emoji: fluxer_types::emoji::ApiEmoji = rest
            .post(&fluxer_types::Routes::guild_emojis(&self.id), Some(&body))
            .await?;
        Ok(emoji)
    }

    pub async fn create_sticker(
        &self,
        rest: &fluxer_rest::Rest,
        body: &Value,
    ) -> crate::Result<fluxer_types::sticker::ApiSticker> {
        let sticker: fluxer_types::sticker::ApiSticker = rest
            .post(&fluxer_types::Routes::guild_stickers(&self.id), Some(body))
            .await?;
        Ok(sticker)
    }

    pub async fn bulk_create_emojis(
        &self,
        rest: &fluxer_rest::Rest,
        emojis: &[(&str, &str)],
    ) -> Vec<fluxer_types::emoji::ApiEmoji> {
        let mut created = Vec::with_capacity(emojis.len());
        for (name, image) in emojis {
            if let Ok(emoji) = self.create_emoji(rest, name, image, None).await {
                created.push(emoji);
            }
        }
        created
    }

    pub fn resolve_role_id(&self, role: &str) -> Option<String> {
        if self.roles.contains_key(role) {
            return Some(role.to_string());
        }
        self.roles
            .values()
            .find(|r| r.name == role)
            .map(|r| r.id.clone())
    }

    pub fn channel_overwrites(&self, _channel_id: &str) -> Vec<ApiChannelOverwrite> {
        Vec::new()
    }

    pub async fn set_text_channel_flexible_names(
        &self,
        rest: &fluxer_rest::Rest,
        enabled: bool,
    ) -> crate::Result<()> {
        let body = serde_json::json!({ "enabled": enabled });
        let _: Value = rest
            .patch(
                &fluxer_types::Routes::guild_text_channel_flexible_names(&self.id),
                Some(&body),
            )
            .await?;
        Ok(())
    }

    pub async fn set_detached_banner(
        &self,
        rest: &fluxer_rest::Rest,
        enabled: bool,
    ) -> crate::Result<()> {
        let body = serde_json::json!({ "enabled": enabled });
        let _: Value = rest
            .patch(
                &fluxer_types::Routes::guild_detached_banner(&self.id),
                Some(&body),
            )
            .await?;
        Ok(())
    }

    pub async fn set_disallow_unclaimed_accounts(
        &self,
        rest: &fluxer_rest::Rest,
        enabled: bool,
    ) -> crate::Result<()> {
        let body = serde_json::json!({ "enabled": enabled });
        let _: Value = rest
            .patch(
                &fluxer_types::Routes::guild_disallow_unclaimed_accounts(&self.id),
                Some(&body),
            )
            .await?;
        Ok(())
    }
}

impl std::fmt::Display for Guild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
