use fluxer_types::channel::ApiChannelPartial;
use fluxer_types::invite::{ApiGuildPartial, ApiInvite};

use crate::structures::user::User;

#[derive(Debug, Clone)]
pub struct Invite {
    pub code: String,
    pub invite_type: u32,
    pub guild: ApiGuildPartial,
    pub channel: ApiChannelPartial,
    pub inviter: Option<User>,
    pub member_count: Option<u32>,
    pub presence_count: Option<u32>,
    pub expires_at: Option<String>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>,
    pub uses: Option<u32>,
    pub max_uses: Option<u32>,
    pub max_age: Option<u32>,
}

impl Invite {
    pub fn from_api(data: &ApiInvite) -> Self {
        Self {
            code: data.code.clone(),
            invite_type: data.kind,
            guild: data.guild.clone(),
            channel: data.channel.clone(),
            inviter: data.inviter.as_ref().map(User::from_api),
            member_count: data.member_count,
            presence_count: data.presence_count,
            expires_at: data.expires_at.clone(),
            temporary: data.temporary,
            created_at: data.created_at.clone(),
            uses: data.uses,
            max_uses: data.max_uses,
            max_age: data.max_age,
        }
    }

    pub fn url(&self) -> String {
        format!("https://fluxer.gg/{}", self.code)
    }

    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::invite(&self.code))
            .await?;
        Ok(())
    }

    pub async fn get_guild(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<fluxer_types::guild::ApiGuild> {
        let guild: fluxer_types::guild::ApiGuild = rest
            .get(&fluxer_types::Routes::guild(&self.guild.id))
            .await?;
        Ok(guild)
    }
}
