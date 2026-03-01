use fluxer_types::emoji::ApiEmoji;
use fluxer_types::Snowflake;

use crate::util::cdn;

/// A custom emoji in a guild.
#[derive(Debug, Clone)]
pub struct GuildEmoji {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub name: String,
    pub animated: bool,
}

impl GuildEmoji {
    pub fn from_api(data: &ApiEmoji, guild_id: &str) -> Self {
        Self {
            id: data.id.clone(),
            guild_id: guild_id.to_string(),
            name: data.name.clone(),
            animated: data.animated,
        }
    }

    /// CDN URL for this emoji.
    pub fn url(&self) -> String {
        cdn::cdn_emoji_url(&self.id, self.animated)
    }

    /// Emoji identifier for reactions (e.g. `name:id`).
    pub fn identifier(&self) -> String {
        format!("{}:{}", self.name, self.id)
    }

    /// Delete this emoji.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_EXPRESSIONS`.
    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::guild_emoji(&self.guild_id, &self.id))
            .await?;
        Ok(())
    }

    /// Edit the name of this emoji.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_EXPRESSIONS`.
    pub async fn edit_name(
        &mut self,
        rest: &fluxer_rest::Rest,
        new_name: &str,
    ) -> crate::Result<()> {
        let body = serde_json::json!({ "name": new_name });
        let data: ApiEmoji = rest
            .patch(
                &fluxer_types::Routes::guild_emoji(&self.guild_id, &self.id),
                Some(&body),
            )
            .await?;
        self.name = data.name;
        Ok(())
    }
}

impl std::fmt::Display for GuildEmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.animated {
            write!(f, "<a:{}:{}>", self.name, self.id)
        } else {
            write!(f, "<:{}:{}>", self.name, self.id)
        }
    }
}
