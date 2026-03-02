use dashmap::DashMap;

use fluxer_types::Snowflake;

use crate::structures::guild_member::GuildMember;

pub struct GuildMemberManager {
    pub cache: DashMap<Snowflake, GuildMember>,
    pub guild_id: Snowflake,
    pub me_id: Option<Snowflake>,
}

impl GuildMemberManager {
    pub fn new(guild_id: &str) -> Self {
        Self {
            cache: DashMap::new(),
            guild_id: guild_id.to_string(),
            me_id: None,
        }
    }

    pub fn resolve(&self, id: &str) -> Option<GuildMember> {
        self.cache.get(id).map(|v| v.clone())
    }

    pub fn set_me(&mut self, user_id: &str) {
        self.me_id = Some(user_id.to_string());
    }

    pub fn me(&self) -> Option<GuildMember> {
        let id = self.me_id.as_deref()?;
        self.resolve(id)
    }

    pub async fn fetch_me(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<fluxer_types::user::ApiGuildMember> {
        let route = format!("/users/@me/guilds/{}/member", self.guild_id);
        let member: fluxer_types::user::ApiGuildMember = rest.get(&route).await?;
        Ok(member)
    }

    pub async fn fetch(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<GuildMember> {
        let data: fluxer_types::user::ApiGuildMember = rest
            .get(&fluxer_types::Routes::guild_member(&self.guild_id, user_id))
            .await?;
        let member = GuildMember::from_api(&data, &self.guild_id);
        self.cache.insert(member.id.clone(), member.clone());
        Ok(member)
    }

    pub async fn list(
        &self,
        rest: &fluxer_rest::Rest,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> crate::Result<Vec<GuildMember>> {
        let mut route = fluxer_types::Routes::guild_members(&self.guild_id);
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
        let data: Vec<fluxer_types::user::ApiGuildMember> = rest.get(&route).await?;
        let members: Vec<GuildMember> = data
            .iter()
            .map(|m| {
                let member = GuildMember::from_api(m, &self.guild_id);
                self.cache.insert(member.id.clone(), member.clone());
                member
            })
            .collect();
        Ok(members)
    }
}
