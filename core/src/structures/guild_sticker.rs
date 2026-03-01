use fluxer_types::sticker::ApiSticker;
use fluxer_types::Snowflake;

use crate::util::cdn;

/// A custom sticker in a guild.
#[derive(Debug, Clone)]
pub struct GuildSticker {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub animated: bool,
}

impl GuildSticker {
    pub fn from_api(data: &ApiSticker, guild_id: &str) -> Self {
        Self {
            id: data.id.clone(),
            guild_id: guild_id.to_string(),
            name: data.name.clone(),
            description: data.description.clone(),
            tags: data.tags.clone(),
            animated: data.animated,
        }
    }

    /// CDN URL for this sticker.
    pub fn url(&self) -> String {
        cdn::cdn_sticker_url(&self.id, self.animated)
    }

    /// Delete this sticker.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_EXPRESSIONS`.
    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_sticker(&self.guild_id, &self.id))
            .await?;
        Ok(())
    }

    /// Edit this sticker.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_EXPRESSIONS`.
    pub async fn edit(
        &mut self,
        rest: &fluxer_rest::Rest,
        body: &serde_json::Value,
    ) -> crate::Result<()> {
        let data: ApiSticker = rest
            .patch(
                &fluxer_types::Routes::guild_sticker(&self.guild_id, &self.id),
                Some(body),
            )
            .await?;
        self.name = data.name;
        self.description = data.description;
        self.tags = data.tags;
        Ok(())
    }
}
