use fluxer_types::{ApiEmbed, ApiMessageReference};
use serde::{Deserialize, Serialize};

use crate::embed::EmbedBuilder;
use crate::attachment::AttachmentPayload;

const CONTENT_MAX: usize = 2000;
const EMBEDS_MAX: usize = 10;

/// Serializable message payload for the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePayloadData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<ApiEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentPayload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<ApiMessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
}

/// Builder for message payloads.
#[derive(Debug, Clone, Default)]
pub struct MessagePayload {
    data: MessagePayloadData,
}

impl MessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set message text. Max 2000 characters.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        let c = content.into();
        assert!(c.len() <= CONTENT_MAX, "content must be <= {CONTENT_MAX} characters");
        self.data.content = Some(c);
        self
    }

    /// Set embeds. Max 10. Replaces existing.
    pub fn embeds(mut self, embeds: Vec<ApiEmbed>) -> Self {
        assert!(embeds.len() <= EMBEDS_MAX, "embeds must be <= {EMBEDS_MAX}");
        self.data.embeds = Some(embeds);
        self
    }

    /// Add one embed. Max 10 total.
    pub fn add_embed(mut self, embed: ApiEmbed) -> Self {
        let list = self.data.embeds.get_or_insert_with(Vec::new);
        assert!(list.len() < EMBEDS_MAX, "embeds must be <= {EMBEDS_MAX}");
        list.push(embed);
        self
    }

    /// Add one embed from an `EmbedBuilder`.
    pub fn add_embed_builder(self, builder: EmbedBuilder) -> Self {
        self.add_embed(builder.build())
    }

    /// Set attachment metadata.
    pub fn attachments(mut self, attachments: Vec<AttachmentPayload>) -> Self {
        self.data.attachments = Some(attachments);
        self
    }

    /// Set a reply reference.
    pub fn reply(mut self, channel_id: impl Into<String>, message_id: impl Into<String>, guild_id: Option<String>) -> Self {
        self.data.message_reference = Some(ApiMessageReference {
            channel_id: channel_id.into(),
            message_id: message_id.into(),
            guild_id,
            kind: None,
        });
        self
    }

    /// Enable text-to-speech.
    pub fn tts(mut self, tts: bool) -> Self {
        self.data.tts = Some(tts);
        self
    }

    /// Set message flags.
    pub fn flags(mut self, flags: u32) -> Self {
        self.data.flags = Some(flags);
        self
    }

    /// Build the payload data.
    pub fn build(self) -> MessagePayloadData {
        self.data
    }

    /// Create from a string (content only).
    pub fn from_content(content: impl Into<String>) -> Self {
        Self::new().content(content)
    }
}
