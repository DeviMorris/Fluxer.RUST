use fluxer_types::channel::{ApiChannel, ChannelType};
use fluxer_types::Snowflake;

use super::typed_channel::TypedChannel;

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

    pub fn from_id(id: impl Into<Snowflake>) -> Self {
        Self {
            id: id.into(),
            kind: 0,
            guild_id: None,
            name: None,
            topic: None,
            url: None,
            icon: None,
            owner_id: None,
            position: None,
            parent_id: None,
            bitrate: None,
            user_limit: None,
            rtc_region: None,
            last_message_id: None,
            nsfw: false,
            rate_limit_per_user: None,
            permission_overwrites: Vec::new(),
        }
    }

    pub fn is_text(&self) -> bool {
        self.kind == ChannelType::GuildText as u16 || self.kind == ChannelType::Dm as u16
    }

    pub fn is_voice(&self) -> bool {
        self.kind == ChannelType::GuildVoice as u16
    }

    pub fn is_category(&self) -> bool {
        self.kind == ChannelType::GuildCategory as u16
    }

    pub fn is_dm(&self) -> bool {
        self.kind == ChannelType::Dm as u16
    }

    pub fn is_guild(&self) -> bool {
        self.guild_id.is_some()
    }

    pub fn as_typed(&self) -> TypedChannel<'_> {
        TypedChannel::from(self)
    }

    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("unknown")
    }

    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }

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

    pub async fn send_files(
        &self,
        rest: &fluxer_rest::Rest,
        payload: &fluxer_builders::MessagePayloadData,
        files: &[fluxer_builders::FileAttachment],
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        let form = fluxer_builders::build_multipart_form(payload, files);
        let msg: fluxer_types::message::ApiMessage = rest
            .post_multipart(
                &fluxer_types::Routes::channel_messages(&self.id),
                form,
            )
            .await?;
        Ok(msg)
    }

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

    pub async fn send_typing(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        let _: serde_json::Value = rest
            .post(
                &fluxer_types::Routes::channel_typing(&self.id),
                Option::<&()>::None,
            )
            .await?;
        Ok(())
    }

    pub async fn bulk_delete_messages(
        &self,
        rest: &fluxer_rest::Rest,
        message_ids: &[String],
    ) -> crate::Result<()> {
        let body = serde_json::json!({ "message_ids": message_ids });
        let _: serde_json::Value = rest
            .post(
                &fluxer_types::Routes::channel_bulk_delete(&self.id),
                Some(&body),
            )
            .await?;
        Ok(())
    }

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

    pub async fn fetch_webhooks(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::webhook::ApiWebhook>> {
        let whs: Vec<fluxer_types::webhook::ApiWebhook> = rest
            .get(&fluxer_types::Routes::channel_webhooks(&self.id))
            .await?;
        Ok(whs)
    }

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

    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel(&self.id))
            .await?;
        Ok(())
    }

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

    pub async fn unpin_message(
        &self,
        rest: &fluxer_rest::Rest,
        message_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_pin(&self.id, message_id))
            .await?;
        Ok(())
    }

    pub async fn fetch_invites(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::invite::ApiInvite>> {
        let invites: Vec<fluxer_types::invite::ApiInvite> = rest
            .get(&fluxer_types::Routes::channel_invites(&self.id))
            .await?;
        Ok(invites)
    }

    pub async fn edit_permission(
        &self,
        rest: &fluxer_rest::Rest,
        overwrite_id: &str,
        body: &serde_json::Value,
    ) -> crate::Result<()> {
        let _: serde_json::Value = rest
            .put(
                &fluxer_types::Routes::channel_permission(&self.id, overwrite_id),
                Some(body),
            )
            .await?;
        Ok(())
    }

    pub async fn delete_permission(
        &self,
        rest: &fluxer_rest::Rest,
        overwrite_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_permission(
            &self.id,
            overwrite_id,
        ))
        .await?;
        Ok(())
    }

    pub async fn fetch_pinned_messages(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::message::ApiMessage>> {
        let msgs: Vec<fluxer_types::message::ApiMessage> = rest
            .get(&fluxer_types::Routes::channel_pins(&self.id))
            .await?;
        Ok(msgs)
    }

    pub async fn fetch_messages(
        &self,
        rest: &fluxer_rest::Rest,
        limit: Option<u32>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> crate::Result<Vec<fluxer_types::message::ApiMessage>> {
        let mut route = fluxer_types::Routes::channel_messages(&self.id);
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(b) = before {
            params.push(format!("before={b}"));
        }
        if let Some(a) = after {
            params.push(format!("after={a}"));
        }
        if !params.is_empty() {
            route = format!("{route}?{}", params.join("&"));
        }
        let msgs: Vec<fluxer_types::message::ApiMessage> = rest.get(&route).await?;
        Ok(msgs)
    }

    pub async fn add_recipient(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<()> {
        let _: serde_json::Value = rest
            .put(
                &fluxer_types::Routes::channel_recipient(&self.id, user_id),
                Option::<&()>::None,
            )
            .await?;
        Ok(())
    }

    pub async fn remove_recipient(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_recipient(&self.id, user_id))
            .await?;
        Ok(())
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<#{}>", self.id)
    }
}
