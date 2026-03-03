use reqwest::multipart::{Form, Part};

use crate::attachment::AttachmentPayload;
use crate::message::MessagePayloadData;

#[derive(Debug, Clone)]
pub struct FileAttachment {
    pub name: String,
    pub data: Vec<u8>,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub spoiler: bool,
}

impl FileAttachment {
    pub fn new(name: impl Into<String>, data: Vec<u8>) -> Self {
        let name = name.into();
        assert!(!name.trim().is_empty(), "file name must not be empty");
        assert!(!data.is_empty(), "file data must not be empty");
        Self {
            name,
            data,
            content_type: None,
            description: None,
            spoiler: false,
        }
    }

    pub fn content_type(mut self, mime: impl Into<String>) -> Self {
        self.content_type = Some(mime.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }

    pub fn filename(&self) -> String {
        if self.spoiler && !self.name.starts_with("SPOILER_") {
            format!("SPOILER_{}", self.name)
        } else {
            self.name.clone()
        }
    }
}

pub fn build_multipart_form(payload: &MessagePayloadData, files: &[FileAttachment]) -> Form {
    let attachment_meta: Vec<AttachmentPayload> = files
        .iter()
        .enumerate()
        .map(|(i, f)| AttachmentPayload {
            id: i as u32,
            filename: f.filename(),
            description: f.description.clone(),
        })
        .collect();

    let mut payload_clone = payload.clone();
    if payload_clone.attachments.is_none()
        || payload_clone
            .attachments
            .as_ref()
            .is_some_and(|a| a.is_empty())
    {
        payload_clone.attachments = Some(attachment_meta);
    }

    let json_str =
        serde_json::to_string(&payload_clone).expect("MessagePayloadData is always serializable");

    let mut form = Form::new().text("payload_json", json_str);

    for (i, file) in files.iter().enumerate() {
        let mime = file
            .content_type
            .clone()
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let part = Part::bytes(file.data.clone())
            .file_name(file.filename())
            .mime_str(&mime)
            .expect("valid MIME type");

        form = form.part(format!("files[{i}]"), part);
    }

    form
}
