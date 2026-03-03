use fluxer_types::Snowflake;
use fluxer_types::channel::OverwriteType;
use fluxer_types::user::ApiGuildMember;

use crate::structures::channel::Channel;
use crate::structures::user::User;
use crate::util::cdn::{self, CdnOptions};

#[derive(Debug, Clone)]
pub struct GuildMember {
    pub id: Snowflake,
    pub user: User,
    pub guild_id: Snowflake,
    pub nick: Option<String>,
    pub role_ids: Vec<Snowflake>,
    pub joined_at: String,
    pub communication_disabled_until: Option<String>,
    pub mute: bool,
    pub deaf: bool,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub profile_flags: Option<u32>,
    pub premium_since: Option<String>,
}

impl GuildMember {
    pub fn from_api(data: &ApiGuildMember, guild_id: &str) -> Self {
        let (id, user) = match &data.user {
            Some(u) => (u.id.clone(), User::from_api(u)),
            None => (String::new(), User::unknown()),
        };
        Self {
            id,
            user,
            guild_id: guild_id.to_string(),
            nick: data.nick.clone(),
            role_ids: data.roles.clone(),
            joined_at: data.joined_at.clone(),
            communication_disabled_until: data.communication_disabled_until.clone(),
            mute: data.mute.unwrap_or(false),
            deaf: data.deaf.unwrap_or(false),
            avatar: data.avatar.clone(),
            banner: data.banner.clone(),
            accent_color: data.accent_color,
            profile_flags: data.profile_flags,
            premium_since: data.premium_since.clone(),
        }
    }

    pub fn display_name(&self) -> &str {
        self.nick
            .as_deref()
            .or(self.user.global_name.as_deref())
            .unwrap_or(&self.user.username)
    }

    pub fn avatar_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_member_avatar_url(&self.guild_id, &self.id, self.avatar.as_deref(), opts)
    }

    pub fn display_avatar_url(&self, opts: &CdnOptions) -> String {
        self.avatar_url(opts)
            .unwrap_or_else(|| self.user.display_avatar_url(opts))
    }

    pub fn banner_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_member_banner_url(&self.guild_id, &self.id, self.banner.as_deref(), opts)
    }

    pub async fn add_role(&self, rest: &fluxer_rest::Rest, role_id: &str) -> crate::Result<()> {
        rest.put_empty(&fluxer_types::Routes::guild_member_role(
            &self.guild_id,
            &self.id,
            role_id,
        ))
        .await?;
        Ok(())
    }

    pub async fn remove_role(&self, rest: &fluxer_rest::Rest, role_id: &str) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_member_role(
            &self.guild_id,
            &self.id,
            role_id,
        ))
        .await?;
        Ok(())
    }

    pub async fn edit(
        &self,
        rest: &fluxer_rest::Rest,
        body: &serde_json::Value,
    ) -> crate::Result<ApiGuildMember> {
        let data: ApiGuildMember = rest
            .patch(
                &fluxer_types::Routes::guild_member(&self.guild_id, &self.id),
                Some(body),
            )
            .await?;
        Ok(data)
    }

    pub fn mention(&self) -> String {
        self.user.mention()
    }

    pub fn has_role(&self, role_id: &str) -> bool {
        self.role_ids.iter().any(|r| r == role_id)
    }

    pub fn permissions(
        &self,
        guild_roles: &std::collections::HashMap<String, crate::structures::role::Role>,
    ) -> fluxer_util::Permissions {
        let mut perms = fluxer_util::Permissions::empty();
        for role in guild_roles.values() {
            let is_everyone = role.name == "@everyone" || role.id == self.guild_id;
            if is_everyone || self.role_ids.iter().any(|r| r == &role.id) {
                perms |= role.permissions();
            }
        }
        if perms.contains(fluxer_util::Permissions::ADMINISTRATOR) {
            return fluxer_util::Permissions::all();
        }
        perms
    }

    pub fn permissions_in(
        &self,
        channel: &Channel,
        guild_roles: &std::collections::HashMap<String, crate::structures::role::Role>,
    ) -> fluxer_util::Permissions {
        let base = self.permissions(guild_roles);
        if base.contains(fluxer_util::Permissions::ADMINISTRATOR) {
            return fluxer_util::Permissions::all();
        }

        let mut perms = base;

        for ow in &channel.permission_overwrites {
            if ow.kind == OverwriteType::Role && ow.id == self.guild_id {
                let deny = fluxer_util::parse_permissions(&ow.deny);
                let allow = fluxer_util::parse_permissions(&ow.allow);
                perms.remove(deny);
                perms.insert(allow);
            }
        }

        let mut role_allow = fluxer_util::Permissions::empty();
        let mut role_deny = fluxer_util::Permissions::empty();
        for ow in &channel.permission_overwrites {
            if ow.kind == OverwriteType::Role
                && ow.id != self.guild_id
                && self.role_ids.iter().any(|r| r == &ow.id)
            {
                role_deny |= fluxer_util::parse_permissions(&ow.deny);
                role_allow |= fluxer_util::parse_permissions(&ow.allow);
            }
        }
        perms.remove(role_deny);
        perms.insert(role_allow);

        for ow in &channel.permission_overwrites {
            if ow.kind == OverwriteType::Member && ow.id == self.id {
                let deny = fluxer_util::parse_permissions(&ow.deny);
                let allow = fluxer_util::parse_permissions(&ow.allow);
                perms.remove(deny);
                perms.insert(allow);
            }
        }

        perms
    }

    pub async fn kick(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_member(
            &self.guild_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

    pub async fn ban(&self, rest: &fluxer_rest::Rest, reason: Option<&str>) -> crate::Result<()> {
        let body = reason.map(|r| serde_json::json!({ "reason": r }));
        let _: serde_json::Value = rest
            .put(
                &fluxer_types::Routes::guild_ban(&self.guild_id, &self.id),
                body.as_ref(),
            )
            .await?;
        Ok(())
    }

    pub async fn timeout(
        &self,
        rest: &fluxer_rest::Rest,
        until: Option<&str>,
    ) -> crate::Result<fluxer_types::user::ApiGuildMember> {
        let body = serde_json::json!({
            "communication_disabled_until": until
        });
        let data: fluxer_types::user::ApiGuildMember = rest
            .patch(
                &fluxer_types::Routes::guild_member(&self.guild_id, &self.id),
                Some(&body),
            )
            .await?;
        Ok(data)
    }
}

impl std::fmt::Display for GuildMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@{}>", self.id)
    }
}
