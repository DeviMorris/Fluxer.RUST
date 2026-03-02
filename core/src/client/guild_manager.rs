use dashmap::DashMap;

use fluxer_types::guild::ApiGuild;

use crate::structures::guild::Guild;

pub struct GuildManager<'a> {
    cache: &'a DashMap<String, Guild>,
    rest: &'a fluxer_rest::Rest,
}

impl<'a> GuildManager<'a> {
    pub fn new(cache: &'a DashMap<String, Guild>, rest: &'a fluxer_rest::Rest) -> Self {
        Self { cache, rest }
    }

    pub fn get(&self, id: &str) -> Option<Guild> {
        self.cache.get(id).map(|r| r.clone())
    }

    pub async fn fetch(&self, id: &str) -> crate::Result<Guild> {
        let data: ApiGuild = self
            .rest
            .get(&fluxer_types::Routes::guild(id))
            .await?;
        let guild = Guild::from_api(&data);
        self.cache.insert(guild.id.clone(), guild.clone());
        Ok(guild)
    }

    pub async fn resolve(&self, id: &str) -> crate::Result<Guild> {
        if let Some(g) = self.get(id) {
            return Ok(g);
        }
        self.fetch(id).await
    }
}
