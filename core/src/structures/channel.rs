use fluxer_types::channel::{ApiChannel, ChannelType};
use fluxer_types::Snowflake;

/// Channel on Fluxer — text, voice, category, DM, or link.
///
/// Constructed from API data. Use `kind()` to check the channel type.
#[derive(Debug, Clone)]
pub struct Channel {
    pub id: Snowflake,
    pub kind: u16,
    pub guild_id: Option<Snowflake>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub position: Option<i32>,
    pub parent_id: Option<Snowflake>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rtc_region: Option<String>,
    pub last_message_id: Option<Snowflake>,
    pub nsfw: bool,
    pub rate_limit_per_user: Option<u32>,
    pub permission_overwrites: Vec<fluxer_types::channel::ApiChannelOverwrite>,
}

impl Channel {
    pub fn from_api(data: &ApiChannel) -> Self {
        Self {
            id: data.id.clone(),
            kind: data.kind,
            guild_id: data.guild_id.clone(),
            name: data.name.clone(),
            topic: data.topic.clone(),
            url: data.url.clone(),
            icon: data.icon.clone(),
            owner_id: data.owner_id.clone(),
            position: data.position,
            parent_id: data.parent_id.clone(),
            bitrate: data.bitrate,
            user_limit: data.user_limit,
            rtc_region: data.rtc_region.clone(),
            last_message_id: data.last_message_id.clone(),
            nsfw: data.nsfw.unwrap_or(false),
            rate_limit_per_user: data.rate_limit_per_user,
            permission_overwrites: data.permission_overwrites.clone().unwrap_or_default(),
        }
    }

    /// Whether this is a text channel.
    pub fn is_text(&self) -> bool {
        self.kind == ChannelType::GuildText as u16 || self.kind == ChannelType::Dm as u16
    }

    /// Whether this is a voice channel.
    pub fn is_voice(&self) -> bool {
        self.kind == ChannelType::GuildVoice as u16
    }

    /// Whether this is a category channel.
    pub fn is_category(&self) -> bool {
        self.kind == ChannelType::GuildCategory as u16
    }

    /// Whether this is a DM channel.
    pub fn is_dm(&self) -> bool {
        self.kind == ChannelType::Dm as u16
    }

    /// Whether this is a guild channel (has guild_id).
    pub fn is_guild(&self) -> bool {
        self.guild_id.is_some()
    }

    /// Display name of the channel.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("unknown")
    }

    /// Mention string (e.g. `<#123456>`).
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }

    /// Send a message to this channel.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `SEND_MESSAGES`.
    pub async fn send(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        let msg: fluxer_types::message::ApiMessage = rest
            .post(
                &fluxer_types::Routes::channel_messages(&self.id),
                Some(body),
            )
            .await?;
        Ok(msg)
    }

    /// Fetch a single message by ID.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] on failure.
    pub async fn fetch_message(
        &self,
        rest: &fluxer_rest::Rest,
        message_id: &str,
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        let msg: fluxer_types::message::ApiMessage = rest
            .get(&fluxer_types::Routes::channel_message(&self.id, message_id))
            .await?;
        Ok(msg)
    }

    /// Trigger the typing indicator in this channel.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] on failure.
    pub async fn send_typing(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        let _: serde_json::Value = rest
            .post(
                &fluxer_types::Routes::channel_typing(&self.id),
                Option::<&()>::None,
            )
            .await?;
        Ok(())
    }

    /// Delete messages in bulk. Max 100 per call, messages must be < 14 days old.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_MESSAGES`.
    pub async fn bulk_delete_messages(
        &self,
        rest: &fluxer_rest::Rest,
        message_ids: &[String],
    ) -> crate::Result<()> {
        let body = serde_json::json!({ "messages": message_ids });
        let _: serde_json::Value = rest
            .post(
                &fluxer_types::Routes::channel_bulk_delete(&self.id),
                Some(&body),
            )
            .await?;
        Ok(())
    }

    /// Create a webhook in this channel.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_WEBHOOKS`.
    pub async fn create_webhook(
        &self,
        rest: &fluxer_rest::Rest,
        name: &str,
        avatar: Option<&str>,
    ) -> crate::Result<fluxer_types::webhook::ApiWebhook> {
        let mut body = serde_json::json!({ "name": name });
        if let Some(a) = avatar {
            body["avatar"] = serde_json::Value::String(a.to_string());
        }
        let wh: fluxer_types::webhook::ApiWebhook = rest
            .post(
                &fluxer_types::Routes::channel_webhooks(&self.id),
                Some(&body),
            )
            .await?;
        Ok(wh)
    }

    /// Fetch webhooks for this channel.
    pub async fn fetch_webhooks(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::webhook::ApiWebhook>> {
        let whs: Vec<fluxer_types::webhook::ApiWebhook> = rest
            .get(&fluxer_types::Routes::channel_webhooks(&self.id))
            .await?;
        Ok(whs)
    }

    /// Create an invite for this channel.
    pub async fn create_invite(
        &self,
        rest: &fluxer_rest::Rest,
        max_age: Option<u32>,
        max_uses: Option<u32>,
        temporary: Option<bool>,
    ) -> crate::Result<fluxer_types::invite::ApiInvite> {
        let mut body = serde_json::Map::new();
        if let Some(v) = max_age {
            body.insert("max_age".into(), serde_json::Value::from(v));
        }
        if let Some(v) = max_uses {
            body.insert("max_uses".into(), serde_json::Value::from(v));
        }
        if let Some(v) = temporary {
            body.insert("temporary".into(), serde_json::Value::from(v));
        }
        let body_val = serde_json::Value::Object(body);
        let invite: fluxer_types::invite::ApiInvite = rest
            .post(
                &fluxer_types::Routes::channel_invites(&self.id),
                Some(&body_val),
            )
            .await?;
        Ok(invite)
    }

    /// Edit this channel.
    pub async fn edit(
        &self,
        rest: &fluxer_rest::Rest,
        body: &serde_json::Value,
    ) -> crate::Result<fluxer_types::channel::ApiChannel> {
        let ch: fluxer_types::channel::ApiChannel = rest
            .patch(&fluxer_types::Routes::channel(&self.id), Some(body))
            .await?;
        Ok(ch)
    }

    /// Delete this channel.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_CHANNELS`.
    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel(&self.id))
            .await?;
        Ok(())
    }

    /// Pin a message in this channel.
    pub async fn pin_message(
        &self,
        rest: &fluxer_rest::Rest,
        message_id: &str,
    ) -> crate::Result<()> {
        let _: serde_json::Value = rest
            .put(
                &fluxer_types::Routes::channel_pin(&self.id, message_id),
                Option::<&()>::None,
            )
            .await?;
        Ok(())
    }

    /// Unpin a message in this channel.
    pub async fn unpin_message(
        &self,
        rest: &fluxer_rest::Rest,
        message_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_pin(&self.id, message_id))
            .await?;
        Ok(())
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<#{}>", self.id)
    }
}
