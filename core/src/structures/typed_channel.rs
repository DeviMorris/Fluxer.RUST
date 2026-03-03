use super::channel::Channel;

#[derive(Debug)]
pub enum TypedChannel<'a> {
    Text(TextChannel<'a>),
    Voice(VoiceChannel<'a>),
    Category(CategoryChannel<'a>),
    Dm(DmChannel<'a>),
    Unknown(&'a Channel),
}

impl<'a> From<&'a Channel> for TypedChannel<'a> {
    fn from(ch: &'a Channel) -> Self {
        if ch.is_voice() {
            TypedChannel::Voice(VoiceChannel(ch))
        } else if ch.is_category() {
            TypedChannel::Category(CategoryChannel(ch))
        } else if ch.is_dm() {
            TypedChannel::Dm(DmChannel(ch))
        } else if ch.is_text() {
            TypedChannel::Text(TextChannel(ch))
        } else {
            TypedChannel::Unknown(ch)
        }
    }
}

#[derive(Debug)]
pub struct TextChannel<'a>(pub &'a Channel);

impl<'a> TextChannel<'a> {
    pub fn inner(&self) -> &Channel {
        self.0
    }

    pub fn topic(&self) -> Option<&str> {
        self.0.topic.as_deref()
    }

    pub fn nsfw(&self) -> bool {
        self.0.nsfw
    }

    pub fn rate_limit_per_user(&self) -> Option<u32> {
        self.0.rate_limit_per_user
    }

    pub fn last_message_id(&self) -> Option<&str> {
        self.0.last_message_id.as_deref()
    }

    pub fn parent_id(&self) -> Option<&str> {
        self.0.parent_id.as_deref()
    }

    pub async fn send(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        self.0.send(rest, body).await
    }

    pub async fn send_files(
        &self,
        rest: &fluxer_rest::Rest,
        payload: &fluxer_builders::MessagePayloadData,
        files: &[fluxer_builders::FileAttachment],
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        self.0.send_files(rest, payload, files).await
    }

    pub async fn send_typing(&self, rest: &fluxer_rest::Rest) -> crate::Result<()> {
        self.0.send_typing(rest).await
    }

    pub async fn bulk_delete_messages(
        &self,
        rest: &fluxer_rest::Rest,
        message_ids: &[String],
    ) -> crate::Result<()> {
        self.0.bulk_delete_messages(rest, message_ids).await
    }

    pub async fn fetch_messages(
        &self,
        rest: &fluxer_rest::Rest,
        limit: Option<u32>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> crate::Result<Vec<fluxer_types::message::ApiMessage>> {
        self.0.fetch_messages(rest, limit, before, after).await
    }

    pub async fn fetch_message(
        &self,
        rest: &fluxer_rest::Rest,
        message_id: &str,
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        self.0.fetch_message(rest, message_id).await
    }

    pub async fn fetch_pinned_messages(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<Vec<fluxer_types::message::ApiMessage>> {
        self.0.fetch_pinned_messages(rest).await
    }
}

impl std::ops::Deref for TextChannel<'_> {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug)]
pub struct VoiceChannel<'a>(pub &'a Channel);

impl<'a> VoiceChannel<'a> {
    pub fn inner(&self) -> &Channel {
        self.0
    }

    pub fn bitrate(&self) -> Option<u32> {
        self.0.bitrate
    }

    pub fn user_limit(&self) -> Option<u32> {
        self.0.user_limit
    }

    pub fn rtc_region(&self) -> Option<&str> {
        self.0.rtc_region.as_deref()
    }

    pub fn parent_id(&self) -> Option<&str> {
        self.0.parent_id.as_deref()
    }
}

impl std::ops::Deref for VoiceChannel<'_> {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug)]
pub struct CategoryChannel<'a>(pub &'a Channel);

impl<'a> CategoryChannel<'a> {
    pub fn inner(&self) -> &Channel {
        self.0
    }

    pub fn position(&self) -> Option<i32> {
        self.0.position
    }
}

impl std::ops::Deref for CategoryChannel<'_> {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug)]
pub struct DmChannel<'a>(pub &'a Channel);

impl<'a> DmChannel<'a> {
    pub fn inner(&self) -> &Channel {
        self.0
    }

    pub fn owner_id(&self) -> Option<&str> {
        self.0.owner_id.as_deref()
    }

    pub fn last_message_id(&self) -> Option<&str> {
        self.0.last_message_id.as_deref()
    }

    pub async fn send(
        &self,
        rest: &fluxer_rest::Rest,
        body: &fluxer_builders::MessagePayloadData,
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        self.0.send(rest, body).await
    }

    pub async fn send_files(
        &self,
        rest: &fluxer_rest::Rest,
        payload: &fluxer_builders::MessagePayloadData,
        files: &[fluxer_builders::FileAttachment],
    ) -> crate::Result<fluxer_types::message::ApiMessage> {
        self.0.send_files(rest, payload, files).await
    }

    pub async fn add_recipient(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<()> {
        self.0.add_recipient(rest, user_id).await
    }

    pub async fn remove_recipient(
        &self,
        rest: &fluxer_rest::Rest,
        user_id: &str,
    ) -> crate::Result<()> {
        self.0.remove_recipient(rest, user_id).await
    }
}

impl std::ops::Deref for DmChannel<'_> {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
