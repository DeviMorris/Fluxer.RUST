use fluxer_types::Snowflake;
use fluxer_types::sticker::ApiSticker;

use crate::util::cdn;

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

    pub fn url(&self) -> String {
        cdn::cdn_sticker_url(&self.id, self.animated)
    }

    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_sticker(
            &self.guild_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

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
