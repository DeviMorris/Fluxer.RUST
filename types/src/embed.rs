use serde::{Deserialize, Serialize};

/// Embed author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmbedAuthor {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub proxy_icon_url: Option<String>,
}

/// Embed footer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmbedFooter {
    pub text: String,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub proxy_icon_url: Option<String>,
}

/// Embed media (image, thumbnail, video, audio).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmbedMedia {
    pub url: String,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub content_hash: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub flags: Option<u32>,
}

/// Embed field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmbedField {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub inline: Option<bool>,
}

/// Rich embed object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEmbed {
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub color: Option<u32>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<ApiEmbedAuthor>,
    #[serde(default)]
    pub image: Option<ApiEmbedMedia>,
    #[serde(default)]
    pub thumbnail: Option<ApiEmbedMedia>,
    #[serde(default)]
    pub footer: Option<ApiEmbedFooter>,
    #[serde(default)]
    pub fields: Option<Vec<ApiEmbedField>>,
    #[serde(default)]
    pub provider: Option<ApiEmbedAuthor>,
    #[serde(default)]
    pub video: Option<ApiEmbedMedia>,
    #[serde(default)]
    pub audio: Option<ApiEmbedMedia>,
    #[serde(default)]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub children: Option<Vec<ApiEmbed>>,
}
