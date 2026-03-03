use fluxer_types::Snowflake;
use fluxer_types::embed::ApiEmbed;
use fluxer_types::message::{
    ApiMessage, ApiMessageAttachment, ApiMessageReaction, ApiMessageReference, ApiMessageSticker,
    MessageType,
};
use serde_json::Value;

use crate::structures::user::User;

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
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
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
    pub member_data: Option<Value>,
}

impl Message {
    pub fn from_api(data: &ApiMessage) -> Self {
        let mentions = data
            .mentions
            .as_ref()
            .map(|arr| arr.iter().map(User::from_api).collect())
            .unwrap_or_default();

        let mention_roles = data.mention_roles.clone().unwrap_or_default();

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
            mentions,
            mention_roles,
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
            member_data: None,
        }
    }

    pub fn from_value(data: &Value) -> Option<Self> {
        let api: ApiMessage = serde_json::from_value(data.clone()).ok()?;
        let mut msg = Self::from_api(&api);
        msg.member_data = data.get("member").cloned();
        Some(msg)
    }

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

    pub async fn send_files(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
        files: &[fluxer_builders::FileAttachment],
    ) -> crate::Result<ApiMessage> {
        let form = fluxer_builders::build_multipart_form(body, files);
        let msg: ApiMessage = rest
            .post_multipart(
                &fluxer_types::Routes::channel_messages(&self.channel_id),
                form,
            )
            .await?;
        Ok(msg)
    }

    pub async fn send_to(
        &self,
        rest: &fluxer_rest::Rest,
        channel_id: &str,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = rest
            .post(
                &fluxer_types::Routes::channel_messages(channel_id),
                Some(body),
            )
            .await?;
        Ok(msg)
    }

    pub async fn reply(
        &self,
        rest: &fluxer_rest::Rest,
        content: &str,
    ) -> crate::Result<ApiMessage> {
        let payload = fluxer_builders::MessagePayload::new()
            .content(content)
            .reply(&self.channel_id, &self.id, self.guild_id.clone())
            .build();
        let msg: ApiMessage = rest
            .post(
                &fluxer_types::Routes::channel_messages(&self.channel_id),
                Some(&payload),
            )
            .await?;
        Ok(msg)
    }

    pub async fn reply_with(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<ApiMessage> {
        let mut payload = body.clone();
        payload.message_reference = Some(fluxer_types::message::ApiMessageReference {
            channel_id: self.channel_id.clone(),
            message_id: self.id.clone(),
            guild_id: self.guild_id.clone(),
            kind: None,
        });
        let msg: ApiMessage = rest
            .post(
                &fluxer_types::Routes::channel_messages(&self.channel_id),
                Some(&payload),
            )
            .await?;
        Ok(msg)
    }

    pub async fn reply_with_files(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
        files: &[fluxer_builders::FileAttachment],
    ) -> crate::Result<ApiMessage> {
        let mut payload = body.clone();
        payload.message_reference = Some(fluxer_types::message::ApiMessageReference {
            channel_id: self.channel_id.clone(),
            message_id: self.id.clone(),
            guild_id: self.guild_id.clone(),
            kind: None,
        });
        let form = fluxer_builders::build_multipart_form(&payload, files);
        let msg: ApiMessage = rest
            .post_multipart(
                &fluxer_types::Routes::channel_messages(&self.channel_id),
                form,
            )
            .await?;
        Ok(msg)
    }

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

    pub async fn delete(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_message(
            &self.channel_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

    pub async fn fetch(&self, rest: &fluxer_rest::Rest) -> crate::Result<ApiMessage> {
        let msg: ApiMessage = rest
            .get(&fluxer_types::Routes::channel_message(
                &self.channel_id,
                &self.id,
            ))
            .await?;
        Ok(msg)
    }

    pub async fn add_reaction(&self, rest: &fluxer_rest::Rest, emoji: &str) -> crate::Result<()> {
        let route = format!(
            "{}/@me",
            fluxer_types::Routes::channel_message_reaction(&self.channel_id, &self.id, emoji)
        );
        let _: Value = rest.put(&route, Option::<&()>::None).await?;
        Ok(())
    }

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

    pub async fn remove_user_reaction(
        &self,
        rest: &fluxer_rest::Rest,
        emoji: &str,
        user_id: &str,
    ) -> crate::Result<()> {
        let route = format!(
            "{}/{}",
            fluxer_types::Routes::channel_message_reaction(&self.channel_id, &self.id, emoji),
            user_id
        );
        rest.delete_route(&route).await?;
        Ok(())
    }

    pub async fn remove_all_reactions(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_message_reactions(
            &self.channel_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

    pub async fn remove_reaction_emoji(
        &self,
        rest: &fluxer_rest::Rest,
        emoji: &str,
    ) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_message_reaction(
            &self.channel_id,
            &self.id,
            emoji,
        ))
        .await?;
        Ok(())
    }

    pub async fn fetch_reaction_users(
        &self,
        rest: &fluxer_rest::Rest,
        emoji: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> crate::Result<Vec<fluxer_types::user::ApiUser>> {
        let mut route =
            fluxer_types::Routes::channel_message_reaction(&self.channel_id, &self.id, emoji);
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(a) = after {
            params.push(format!("after={a}"));
        }
        if !params.is_empty() {
            route = format!("{route}?{}", params.join("&"));
        }
        let users: Vec<fluxer_types::user::ApiUser> = rest.get(&route).await?;
        Ok(users)
    }

    pub async fn pin(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        let _: Value = rest
            .put(
                &fluxer_types::Routes::channel_pin_message(&self.channel_id, &self.id),
                Option::<&()>::None,
            )
            .await?;
        Ok(())
    }

    pub async fn unpin(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        rest.delete_route(&fluxer_types::Routes::channel_pin_message(
            &self.channel_id,
            &self.id,
        ))
        .await?;
        Ok(())
    }

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

    pub fn mention_author(&self) -> String {
        self.author.mention()
    }
}

#[derive(Debug, Clone)]
pub struct PartialMessage {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub content: Option<String>,
    pub author_id: Option<Snowflake>,
}

impl PartialMessage {
    pub fn from_value(data: &Value) -> Option<Self> {
        Some(Self {
            id: data.get("id")?.as_str()?.to_string(),
            channel_id: data.get("channel_id")?.as_str()?.to_string(),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .map(String::from),
            author_id: data
                .get("author")
                .and_then(|a| a.get("id"))
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
