use fluxer_types::user::ApiGuildMember;
use fluxer_types::Snowflake;

use crate::structures::user::User;
use crate::util::cdn::{self, CdnOptions};

/// A member of a guild.
///
/// Wraps a `User` with guild-specific data like nickname, roles, and join date.
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
        Self {
            id: data.user.id.clone(),
            user: User::from_api(&data.user),
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

    /// Nickname, global name, or username.
    pub fn display_name(&self) -> &str {
        self.nick
            .as_deref()
            .or(self.user.global_name.as_deref())
            .unwrap_or(&self.user.username)
    }

    /// Guild-specific avatar URL.
    ///
    /// Returns `None` if the member has no guild avatar.
    pub fn avatar_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_member_avatar_url(&self.guild_id, &self.id, self.avatar.as_deref(), opts)
    }

    /// Avatar URL to display (guild avatar → user avatar → default).
    pub fn display_avatar_url(&self, opts: &CdnOptions) -> String {
        self.avatar_url(opts)
            .unwrap_or_else(|| self.user.display_avatar_url(opts))
    }

    /// Guild-specific banner URL.
    ///
    /// Returns `None` if the member has no guild banner.
    pub fn banner_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_member_banner_url(&self.guild_id, &self.id, self.banner.as_deref(), opts)
    }

    /// Add a role to this member.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn add_role(
        &self,
        rest: &fluxer_rest::Rest,
        role_id: &str,
    ) -> crate::Result<()> {
        rest.put_empty(&fluxer_types::Routes::guild_member_role(
            &self.guild_id,
            &self.id,
            role_id,
        ))
        .await?;
        Ok(())
    }

    /// Remove a role from this member.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_ROLES`.
    pub async fn remove_role(
        &self,
        rest: &fluxer_rest::Rest,
        role_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_member_role(
            &self.guild_id,
            &self.id,
            role_id,
        ))
        .await?;
        Ok(())
    }

    /// Edit this guild member.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] on failure.
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

    /// Mention string for this member (e.g. `<@123456>`).
    pub fn mention(&self) -> String {
        self.user.mention()
    }
}

impl std::fmt::Display for GuildMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@{}>", self.id)
    }
}
