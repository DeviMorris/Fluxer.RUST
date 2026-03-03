use fluxer_types::Snowflake;
use fluxer_types::webhook::ApiWebhook;

use crate::structures::user::User;
use crate::util::cdn::{self, CdnOptions};

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

    pub fn avatar_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_avatar_url(&self.id, self.avatar.as_deref(), opts)
    }

    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::webhook(&self.id))
            .await?;
        Ok(())
    }

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
            format!(
                "{}?wait=true",
                fluxer_types::Routes::webhook_execute(&self.id, token)
            )
        } else {
            fluxer_types::Routes::webhook_execute(&self.id, token)
        };
        if wait {
            let msg: fluxer_types::message::ApiMessage = rest.post(&route, Some(body)).await?;
            Ok(Some(msg))
        } else {
            let _: serde_json::Value = rest.post(&route, Some(body)).await?;
            Ok(None)
        }
    }

    pub async fn send_files(
        &self,
        rest: &fluxer_rest::Rest,
        payload: &fluxer_builders::MessagePayloadData,
        files: &[fluxer_builders::FileAttachment],
        wait: bool,
    ) -> crate::Result<Option<fluxer_types::message::ApiMessage>> {
        let token = self
            .token
            .as_deref()
            .ok_or(crate::Error::WebhookTokenRequired)?;
        let route = if wait {
            format!(
                "{}?wait=true",
                fluxer_types::Routes::webhook_execute(&self.id, token)
            )
        } else {
            fluxer_types::Routes::webhook_execute(&self.id, token)
        };
        let form = fluxer_builders::build_multipart_form(payload, files);
        if wait {
            let msg: fluxer_types::message::ApiMessage = rest.post_multipart(&route, form).await?;
            Ok(Some(msg))
        } else {
            let _: serde_json::Value = rest.post_multipart(&route, form).await?;
            Ok(None)
        }
    }

    pub async fn fetch(rest: &fluxer_rest::Rest, webhook_id: &str) -> crate::Result<Webhook> {
        let data: ApiWebhook = rest.get(&fluxer_types::Routes::webhook(webhook_id)).await?;
        Ok(Webhook::from_api(&data))
    }
}
