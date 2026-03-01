use dashmap::DashMap;

use fluxer_types::guild::ApiGuild;

use crate::structures::guild::Guild;

/// Manages cached guilds.
///
/// # Examples
/// ```rust,ignore
/// let guild = client.guilds.get("123456789");
/// ```
pub struct GuildManager<'a> {
    cache: &'a DashMap<String, Guild>,
    rest: &'a fluxer_rest::Rest,
}

impl<'a> GuildManager<'a> {
    pub fn new(cache: &'a DashMap<String, Guild>, rest: &'a fluxer_rest::Rest) -> Self {
        Self { cache, rest }
    }

    /// Get a cached guild by ID.
    pub fn get(&self, id: &str) -> Option<Guild> {
        self.cache.get(id).map(|r| r.clone())
    }

    /// Fetch a guild from the API.
    pub async fn fetch(&self, id: &str) -> crate::Result<Guild> {
        let data: ApiGuild = self
            .rest
            .get(&fluxer_types::Routes::guild(id))
            .await?;
        let guild = Guild::from_api(&data);
        self.cache.insert(guild.id.clone(), guild.clone());
        Ok(guild)
    }

    /// Resolve a guild — from cache or API.
    pub async fn resolve(&self, id: &str) -> crate::Result<Guild> {
        if let Some(g) = self.get(id) {
            return Ok(g);
        }
        self.fetch(id).await
    }
}
