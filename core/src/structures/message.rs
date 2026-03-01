use fluxer_types::embed::ApiEmbed;
use fluxer_types::message::{
    ApiMessage, ApiMessageAttachment, ApiMessageReaction, ApiMessageReference, ApiMessageSticker,
    MessageType,
};
use fluxer_types::Snowflake;

use crate::structures::user::User;

/// A message in a channel.
///
/// Constructed from API data. Methods that require network calls
/// take `&Rest` explicitly.
#[derive(Debug, Clone)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub pinned: bool,
    pub tts: bool,
    pub mention_everyone: bool,
    pub embeds: Vec<ApiEmbed>,
    pub attachments: Vec<ApiMessageAttachment>,
    pub stickers: Vec<ApiMessageSticker>,
    pub reactions: Vec<ApiMessageReaction>,
    pub message_reference: Option<ApiMessageReference>,
    pub referenced_message: Option<Box<Message>>,
    pub message_type: MessageType,
    pub flags: Option<u32>,
    pub nonce: Option<String>,
    pub webhook_id: Option<Snowflake>,
}

impl Message {
    pub fn from_api(data: &ApiMessage) -> Self {
        Self {
            id: data.id.clone(),
            channel_id: data.channel_id.clone(),
            guild_id: data.guild_id.clone(),
            author: User::from_api(&data.author),
            content: data.content.clone(),
            timestamp: data.timestamp.clone(),
            edited_timestamp: data.edited_timestamp.clone(),
            pinned: data.pinned,
            tts: data.tts.unwrap_or(false),
            mention_everyone: data.mention_everyone.unwrap_or(false),
            embeds: data.embeds.clone().unwrap_or_default(),
            attachments: data.attachments.clone().unwrap_or_default(),
            stickers: data.stickers.clone().unwrap_or_default(),
            reactions: data.reactions.clone().unwrap_or_default(),
            message_reference: data.message_reference.clone(),
            referenced_message: data
                .referenced_message
                .as_ref()
                .map(|m| Box::new(Message::from_api(m))),
            message_type: data.kind,
            flags: data.flags,
            nonce: data.nonce.clone(),
            webhook_id: data.webhook_id.clone(),
        }
    }

    /// Send a new message to the same channel.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `SEND_MESSAGES`.
    pub async fn send(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = rest
            .post(
                &fluxer_types::Routes::channel_messages(&self.channel_id),
                Some(body),
            )
            .await?;
        Ok(msg)
    }

    /// Reply to this message.
    ///
    /// Automatically sets `message_reference` to this message.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `SEND_MESSAGES`.
    pub async fn reply(
        &self,
        rest: &fluxer_rest::Rest,
        content: &str,
    ) -> crate::Result<ApiMessage> {
        let payload = fluxer_builders::MessagePayload::new()
            .content(content)
            .reply(
                &self.channel_id,
                &self.id,
                self.guild_id.clone(),
            )
            .build();
        let msg: ApiMessage = rest
            .post(
                &fluxer_types::Routes::channel_messages(&self.channel_id),
                Some(&payload),
            )
            .await?;
        Ok(msg)
    }

    /// Edit this message.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot is not the author.
    pub async fn edit(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = rest
            .patch(
                &fluxer_types::Routes::channel_message(&self.channel_id, &self.id),
                Some(body),
            )
            .await?;
        Ok(msg)
    }

    /// Delete this message.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_MESSAGES` (for others' messages).
    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_message(
            &self.channel_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

    /// Fetch a fresh copy of this message from the API.
    pub async fn fetch(&self, rest: &fluxer_rest::Rest) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = rest
            .get(&fluxer_types::Routes::channel_message(
                &self.channel_id,
                &self.id,
            ))
            .await?;
        Ok(msg)
    }

    /// Add a reaction to this message.
    ///
    /// `emoji` is a URL-encoded emoji string (e.g. `"👍"` or `"name:id"`).
    pub async fn add_reaction(
        &self,
        rest: &fluxer_rest::Rest,
        emoji: &str,
    ) -> crate::Result<()> {
        let route = format!(
            "{}/@me",
            fluxer_types::Routes::channel_message_reaction(&self.channel_id, &self.id, emoji)
        );
        let _: serde_json::Value = rest.put(&route, Option::<&()>::None).await?;
        Ok(())
    }

    /// Remove the bot's reaction from this message.
    pub async fn remove_reaction(
        &self,
        rest: &fluxer_rest::Rest,
        emoji: &str,
    ) -> crate::Result<()> {
        let route = format!(
            "{}/@me",
            fluxer_types::Routes::channel_message_reaction(&self.channel_id, &self.id, emoji)
        );
        rest.delete_route(&route).await?;
        Ok(())
    }

    /// Pin this message.
    pub async fn pin(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        let _: serde_json::Value = rest
            .put(
                &fluxer_types::Routes::channel_pin_message(&self.channel_id, &self.id),
                Option::<&()>::None,
            )
            .await?;
        Ok(())
    }

    /// Unpin this message.
    pub async fn unpin(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_pin_message(
            &self.channel_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

    /// Delete a specific attachment from this message.
    pub async fn delete_attachment(
        &self,
        rest: &fluxer_rest::Rest,
        attachment_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_message_attachment(
            &self.channel_id,
            &self.id,
            attachment_id,
        ))
        .await?;
        Ok(())
    }

    /// Mention string for replying context.
    pub fn mention_author(&self) -> String {
        self.author.mention()
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
