use fluxer_types::user::ApiUser;
use fluxer_types::Snowflake;

use crate::util::cdn::{self, CdnOptions};

/// A user (or bot) on Fluxer.
///
/// Constructed from API data. Contains identity, avatar, and display info.
/// Methods that require network calls take `&Rest` explicitly.
#[derive(Debug, Clone)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: bool,
    pub avatar_color: Option<u32>,
    pub flags: Option<u32>,
    pub system: bool,
    pub banner: Option<String>,
}

impl User {
    pub fn from_api(data: &ApiUser) -> Self {
        Self {
            id: data.id.clone(),
            username: data.username.clone(),
            discriminator: data.discriminator.clone(),
            global_name: data.global_name.clone(),
            avatar: data.avatar.clone(),
            bot: data.bot.unwrap_or(false),
            avatar_color: data.avatar_color,
            flags: data.flags.or(data.public_flags),
            system: data.system.unwrap_or(false),
            banner: data.banner.clone(),
        }
    }

    /// Update mutable fields from fresh API data.
    pub fn patch(&mut self, data: &ApiUser) {
        self.username.clone_from(&data.username);
        self.discriminator.clone_from(&data.discriminator);
        self.global_name.clone_from(&data.global_name);
        self.avatar.clone_from(&data.avatar);
        if let Some(c) = data.avatar_color {
            self.avatar_color = Some(c);
        }
        if let Some(f) = data.flags {
            self.flags = Some(f);
        }
        if let Some(b) = &data.banner {
            self.banner = Some(b.clone());
        }
    }

    /// Get the URL for this user's avatar.
    ///
    /// Auto-detects animated avatars (hash starting with `a_`) and uses gif.
    /// Returns `None` if the user has no custom avatar.
    pub fn avatar_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_avatar_url(&self.id, self.avatar.as_deref(), opts)
    }

    /// Get the avatar URL to display, falling back to the default avatar.
    pub fn display_avatar_url(&self, opts: &CdnOptions) -> String {
        cdn::cdn_display_avatar_url(&self.id, self.avatar.as_deref(), opts)
    }

    /// Get the URL for this user's banner.
    ///
    /// Returns `None` if the user has no banner.
    pub fn banner_url(&self, opts: &CdnOptions) -> Option<String> {
        cdn::cdn_banner_url(&self.id, self.banner.as_deref(), opts)
    }

    /// Mention string (e.g. `<@123456>`).
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }

    /// Display name: global_name or username.
    pub fn display_name(&self) -> &str {
        self.global_name.as_deref().unwrap_or(&self.username)
    }

    /// Create a DM channel with this user.
    ///
    /// # Errors
    /// Returns [`Error::Rest`] on network failure.
    pub async fn create_dm(
        &self,
        rest: &fluxer_rest::Rest,
    ) -> crate::Result<fluxer_types::channel::ApiChannel> {
        let body = serde_json::json!({ "recipient_id": self.id });
        let ch: fluxer_types::channel::ApiChannel =
            rest.post(fluxer_types::Routes::user_me_channels(), Some(&body))
                .await?;
        Ok(ch)
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@{}>", self.id)
    }
}
