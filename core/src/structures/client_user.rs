use serde_json::Value;

use crate::structures::user::User;

#[derive(Debug, Clone)]
pub struct ClientUser {
    pub base: User,
}

impl ClientUser {
    pub fn from_user(user: User) -> Self {
        Self { base: user }
    }

    pub async fn fetch_guilds(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::guild::ApiGuild>> {
        let guilds: Vec<fluxer_types::guild::ApiGuild> = rest
            .get(fluxer_types::Routes::current_user_guilds())
            .await?;
        Ok(guilds)
    }

    pub async fn leave_guild(&self, rest: &fluxer_rest::Rest, guild_id: &str) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::leave_guild(guild_id))
            .await?;
        Ok(())
    }

    pub async fn edit(&self, rest: &fluxer_rest::Rest, body: &Value) -> crate::Result<Value> {
        let data: Value = rest
            .patch(fluxer_types::Routes::current_user(), Some(body))
            .await?;
        Ok(data)
    }

    pub fn mention(&self) -> String {
        self.base.mention()
    }

    pub fn id(&self) -> &str {
        &self.base.id
    }

    pub fn username(&self) -> &str {
        &self.base.username
    }
}

impl std::ops::Deref for ClientUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
