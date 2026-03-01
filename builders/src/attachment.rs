use serde::{Deserialize, Serialize};

/// Serializable attachment metadata for the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentPayload {
    pub id: u32,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Builder for attachment metadata.
#[derive(Debug, Clone)]
pub struct AttachmentBuilder {
    id: u32,
    filename: String,
    description: Option<String>,
    spoiler: bool,
}

impl AttachmentBuilder {
    /// Create a new attachment builder.
    ///
    /// `id` must match the index in the multipart form data.
    pub fn new(id: u32, filename: impl Into<String>) -> Self {
        let name = filename.into();
        assert!(!name.trim().is_empty(), "filename is required");
        Self {
            id,
            filename: name,
            description: None,
            spoiler: false,
        }
    }

    /// Set the displayed filename.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let n = name.into();
        assert!(!n.trim().is_empty(), "filename is required");
        self.filename = if self.spoiler {
            format!("SPOILER_{n}")
        } else {
            n
        };
        self
    }

    /// Set the attachment description (alt text).
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Mark as spoiler (blurred until clicked).
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        if spoiler && !self.filename.starts_with("SPOILER_") {
            self.filename = format!("SPOILER_{}", self.filename);
        } else if !spoiler && self.filename.starts_with("SPOILER_") {
            self.filename = self.filename[8..].to_string();
        }
        self
    }

    /// Build the `AttachmentPayload`.
    pub fn build(self) -> AttachmentPayload {
        AttachmentPayload {
            id: self.id,
            filename: self.filename,
            description: self.description,
        }
    }
}
