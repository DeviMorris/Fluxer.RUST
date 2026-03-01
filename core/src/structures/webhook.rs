use fluxer_types::webhook::ApiWebhook;
use fluxer_types::Snowflake;

use crate::structures::user::User;
use crate::util::cdn::{self, CdnOptions};

/// A webhook (bot or token-based).
///
/// `token` is only available when the webhook was created; fetched webhooks
/// cannot send messages.
#[derive(Debug, Clone)]
pub struct Webhook {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: String,
    pub avatar: Option<String>,
    pub token: Option<String>,
    pub user: User,
}

impl Webhook {
    pub fn from_api(data: &ApiWebhook) -> Self {
        Self {
            id: data.id.clone(),
            guild_id: data.guild_id.clone(),
            channel_id: data.channel_id.clone(),
            name: data.name.clone(),
            avatar: data.avatar.clone(),
            token: data.token.clone(),
            user: User::from_api(&data.user),
        }
    }

    /// Create a Webhook from an ID and token (e.g. stored webhook URL).
    pub fn from_token(id: &str, token: &str) -> Self {
        Self {
            id: id.to_string(),
            guild_id: String::new(),
            channel_id: String::new(),
            name: "Webhook".to_string(),
            avatar: None,
            token: Some(token.to_string()),
            user: User {
                id: String::new(),
                username: "webhook".to_string(),
                discriminator: "0".to_string(),
                global_name: None,
                avatar: None,
                bot: false,
                avatar_color: None,
                flags: None,
                system: false,
                banner: None,
            },
        }
    }

    /// Get the webhook avatar URL.
    pub fn avatar_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_avatar_url(&self.id, self.avatar.as_deref(), opts)
    }

    /// Delete this webhook.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] if the bot lacks `MANAGE_WEBHOOKS`.
    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::webhook(&self.id))
            .await?;
        Ok(())
    }

    /// Edit this webhook (bot auth: name, avatar, channel_id).
    pub async fn edit(
        &mut self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_types::webhook::WebhookUpdateRequest,
    ) -> crate::Result<()> {
        let data: ApiWebhook = rest
            .patch(&fluxer_types::Routes::webhook(&self.id), Some(body))
            .await?;
        self.name = data.name;
        self.avatar = data.avatar;
        self.channel_id = data.channel_id;
        Ok(())
    }

    /// Send a message via this webhook.
    ///
    /// Requires the webhook token.
    ///
    /// # Errors
    /// Returns [`Error::WebhookTokenRequired`] if no token is set.
    /// Returns [`Error::Rest`] on API failure.
    pub async fn send(
        &self,
        rest: &fluxer_rest::Rest,
        body: &serde_json::Value,
        wait: bool,
    ) -> crate::Result<Option<fluxer_types::message::ApiMessage>> {
        let token = self
            .token
            .as_deref()
            .ok_or(crate::Error::WebhookTokenRequired)?;
        let route = if wait {
            format!("{}?wait=true", fluxer_types::Routes::webhook_execute(&self.id, token))
        } else {
            fluxer_types::Routes::webhook_execute(&self.id, token)
        };
        if wait {
            let msg: fluxer_types::message::ApiMessage =
                rest.post(&route, Some(body)).await?;
            Ok(Some(msg))
        } else {
            let _: serde_json::Value = rest.post(&route, Some(body)).await?;
            Ok(None)
        }
    }

    /// Fetch a webhook by ID using bot auth.
    pub async fn fetch(
        rest: &fluxer_rest::Rest,
        webhook_id: &str,
    ) -> crate::Result<Webhook> {
        let data: ApiWebhook = rest
            .get(&fluxer_types::Routes::webhook(webhook_id))
            .await?;
        Ok(Webhook::from_api(&data))
    }
}
