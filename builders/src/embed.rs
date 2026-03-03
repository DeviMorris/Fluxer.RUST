use fluxer_types::{ApiEmbed, ApiEmbedAuthor, ApiEmbedField, ApiEmbedFooter, ApiEmbedMedia};

const MAX_TITLE: usize = 256;
const MAX_DESCRIPTION: usize = 4096;
const MAX_FIELDS: usize = 25;
const MAX_FIELD_NAME: usize = 256;
const MAX_FIELD_VALUE: usize = 1024;
const MAX_FOOTER_TEXT: usize = 2048;
const MAX_AUTHOR_NAME: usize = 256;
const MAX_TOTAL: usize = 6000;

#[derive(Debug, Clone, Default)]
pub struct EmbedBuilder {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    timestamp: Option<String>,
    author: Option<ApiEmbedAuthor>,
    footer: Option<ApiEmbedFooter>,
    image: Option<ApiEmbedMedia>,
    thumbnail: Option<ApiEmbedMedia>,
    video: Option<ApiEmbedMedia>,
    audio: Option<ApiEmbedMedia>,
    fields: Vec<ApiEmbedField>,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        let t = title.into();
        self.title = Some(truncate_str(&t, MAX_TITLE));
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        let d = desc.into();
        self.description = Some(truncate_str(&d, MAX_DESCRIPTION));
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn color_hex(mut self, hex: &str) -> Self {
        if let Some(c) = fluxer_util::resolve_color(hex) {
            self.color = Some(c);
        }
        self
    }

    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Some(fluxer_util::resolve_color_rgb(r, g, b));
        self
    }

    pub fn timestamp(mut self, ts: impl Into<String>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    pub fn author(
        mut self,
        name: impl Into<String>,
        url: Option<String>,
        icon_url: Option<String>,
    ) -> Self {
        let n = name.into();
        self.author = Some(ApiEmbedAuthor {
            name: Some(truncate_str(&n, MAX_AUTHOR_NAME)),
            url,
            icon_url,
            proxy_icon_url: None,
        });
        self
    }

    pub fn footer(mut self, text: impl Into<String>, icon_url: Option<String>) -> Self {
        let t = text.into();
        self.footer = Some(ApiEmbedFooter {
            text: truncate_str(&t, MAX_FOOTER_TEXT),
            icon_url,
            proxy_icon_url: None,
        });
        self
    }

    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.image = Some(media_from_url(url.into()));
        self
    }

    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.thumbnail = Some(media_from_url(url.into()));
        self
    }

    pub fn video(mut self, url: impl Into<String>) -> Self {
        self.video = Some(media_from_url(url.into()));
        self
    }

    pub fn audio(mut self, url: impl Into<String>) -> Self {
        self.audio = Some(media_from_url(url.into()));
        self
    }

    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        if self.fields.len() < MAX_FIELDS {
            let n = name.into();
            let v = value.into();
            self.fields.push(ApiEmbedField {
                name: truncate_str(&n, MAX_FIELD_NAME),
                value: truncate_str(&v, MAX_FIELD_VALUE),
                inline: Some(inline),
            });
        }
        self
    }

    pub fn build(self) -> ApiEmbed {
        let total = char_count(&self.title)
            + char_count(&self.description)
            + self
                .fields
                .iter()
                .map(|f| f.name.len() + f.value.len())
                .sum::<usize>()
            + self.footer.as_ref().map(|f| f.text.len()).unwrap_or(0)
            + self
                .author
                .as_ref()
                .and_then(|a| a.name.as_ref().map(|n| n.len()))
                .unwrap_or(0);

        assert!(
            total <= MAX_TOTAL,
            "embed total length must be <= {MAX_TOTAL}"
        );

        ApiEmbed {
            kind: Some("rich".to_string()),
            title: self.title,
            description: self.description,
            url: self.url,
            color: self.color,
            timestamp: self.timestamp,
            author: self.author,
            footer: self.footer,
            image: self.image,
            thumbnail: self.thumbnail,
            video: self.video,
            audio: self.audio,
            fields: if self.fields.is_empty() {
                None
            } else {
                Some(self.fields)
            },
            provider: None,
            nsfw: None,
            children: None,
        }
    }

    pub fn from_embed(data: ApiEmbed) -> Self {
        Self {
            title: data.title,
            description: data.description,
            url: data.url,
            color: data.color,
            timestamp: data.timestamp,
            author: data.author,
            footer: data.footer,
            image: data.image,
            thumbnail: data.thumbnail,
            video: data.video,
            audio: data.audio,
            fields: data.fields.unwrap_or_default(),
        }
    }
}

fn media_from_url(url: String) -> ApiEmbedMedia {
    ApiEmbedMedia {
        url,
        proxy_url: None,
        content_type: None,
        content_hash: None,
        width: None,
        height: None,
        description: None,
        placeholder: None,
        duration: None,
        flags: None,
    }
}

fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        s.chars().take(max).collect()
    }
}

fn char_count(s: &Option<String>) -> usize {
    s.as_ref().map(|s| s.len()).unwrap_or(0)
}
