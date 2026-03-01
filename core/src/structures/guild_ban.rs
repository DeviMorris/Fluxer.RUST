use fluxer_types::ban::ApiBan;
use fluxer_types::Snowflake;

use crate::structures::user::User;

/// A ban on a guild.
#[derive(Debug, Clone)]
pub struct GuildBan {
    pub guild_id: Snowflake,
    pub user: User,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
}

impl GuildBan {
    pub fn from_api(data: &ApiBan, guild_id: &str) -> Self {
        Self {
            guild_id: guild_id.to_string(),
            user: User::from_api(&data.user),
            reason: data.reason.clone(),
            expires_at: data.expires_at.clone(),
        }
    }

    /// Unban this user.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `BAN_MEMBERS`.
    pub async fn unban(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_ban(&self.guild_id, &self.user.id))
            .await?;
        Ok(())
    }
}
