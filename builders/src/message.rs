use fluxer_types::{ApiEmbed, ApiMessageReference};
use serde::{Deserialize, Serialize};

use crate::embed::EmbedBuilder;
use crate::attachment::AttachmentPayload;
use crate::file::FileAttachment;

const CONTENT_MAX: usize = 2000;
const EMBEDS_MAX: usize = 10;

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

#[derive(Debug, Clone, Default)]
pub struct MessagePayload {
    data: MessagePayloadData,
    files: Vec<FileAttachment>,
}

impl MessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        let c = content.into();
        assert!(c.len() <= CONTENT_MAX, "content must be <= {CONTENT_MAX} characters");
        self.data.content = Some(c);
        self
    }

    pub fn embeds(mut self, embeds: Vec<ApiEmbed>) -> Self {
        assert!(embeds.len() <= EMBEDS_MAX, "embeds must be <= {EMBEDS_MAX}");
        self.data.embeds = Some(embeds);
        self
    }

    pub fn add_embed(mut self, embed: ApiEmbed) -> Self {
        let list = self.data.embeds.get_or_insert_with(Vec::new);
        assert!(list.len() < EMBEDS_MAX, "embeds must be <= {EMBEDS_MAX}");
        list.push(embed);
        self
    }

    pub fn add_embed_builder(self, builder: EmbedBuilder) -> Self {
        self.add_embed(builder.build())
    }

    pub fn attachments(mut self, attachments: Vec<AttachmentPayload>) -> Self {
        self.data.attachments = Some(attachments);
        self
    }

    pub fn reply(mut self, channel_id: impl Into<String>, message_id: impl Into<String>, guild_id: Option<String>) -> Self {
        self.data.message_reference = Some(ApiMessageReference {
            channel_id: channel_id.into(),
            message_id: message_id.into(),
            guild_id,
            kind: None,
        });
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.data.tts = Some(tts);
        self
    }

    pub fn flags(mut self, flags: u32) -> Self {
        self.data.flags = Some(flags);
        self
    }

    pub fn build(self) -> MessagePayloadData {
        self.data
    }

    pub fn has_files(&self) -> bool {
        !self.files.is_empty()
    }

    pub fn build_with_files(self) -> (MessagePayloadData, Vec<FileAttachment>) {
        (self.data, self.files)
    }

    pub fn build_form(self) -> reqwest::multipart::Form {
        let (data, files) = self.build_with_files();
        crate::file::build_multipart_form(&data, &files)
    }

    pub fn attach_file(mut self, file: FileAttachment) -> Self {
        self.files.push(file);
        self
    }

    pub fn attach_files(mut self, files: impl IntoIterator<Item = FileAttachment>) -> Self {
        self.files.extend(files);
        self
    }

    pub fn from_content(content: impl Into<String>) -> Self {
        Self::new().content(content)
    }
}
