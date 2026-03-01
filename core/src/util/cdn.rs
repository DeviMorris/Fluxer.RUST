pub const CDN_URL: &str = "https://fluxerusercontent.com";
pub const STATIC_CDN_URL: &str = "https://fluxerstatic.com";

/// CDN image format options.
pub struct CdnOptions {
    pub size: Option<u32>,
    pub extension: Option<String>,
}

impl Default for CdnOptions {
    fn default() -> Self {
        Self {
            size: None,
            extension: None,
        }
    }
}

fn get_extension(hash: Option<&str>, opts: &CdnOptions) -> String {
    let ext = opts.extension.as_deref().unwrap_or("png");
    match hash {
        Some(h) if h.starts_with("a_") => "gif".to_string(),
        _ => ext.to_string(),
    }
}

fn append_size(opts: &CdnOptions) -> String {
    match opts.size {
        Some(s) => format!("?size={s}"),
        None => String::new(),
    }
}

/// Build a user avatar URL.
///
/// Returns `None` if `avatar_hash` is `None`.
pub fn cdn_avatar_url(
    user_id: &str,
    avatar_hash: Option<&str>,
    opts: &CdnOptions,
) -> Option<String> {
    let hash = avatar_hash?;
    let ext = get_extension(Some(hash), opts);
    let size = append_size(opts);
    Some(format!("{CDN_URL}/avatars/{user_id}/{hash}.{ext}{size}"))
}

/// Build a user banner URL.
///
/// Returns `None` if `banner_hash` is `None`.
pub fn cdn_banner_url(
    resource_id: &str,
    banner_hash: Option<&str>,
    opts: &CdnOptions,
) -> Option<String> {
    let hash = banner_hash?;
    let ext = get_extension(Some(hash), opts);
    let size = append_size(opts);
    Some(format!(
        "{CDN_URL}/banners/{resource_id}/{hash}.{ext}{size}"
    ))
}

/// Build a guild-specific member avatar URL.
///
/// Returns `None` if `avatar_hash` is `None`.
pub fn cdn_member_avatar_url(
    guild_id: &str,
    user_id: &str,
    avatar_hash: Option<&str>,
    opts: &CdnOptions,
) -> Option<String> {
    let hash = avatar_hash?;
    let ext = get_extension(Some(hash), opts);
    let size = append_size(opts);
    Some(format!(
        "{CDN_URL}/guilds/{guild_id}/users/{user_id}/avatars/{hash}.{ext}{size}"
    ))
}

/// Build a guild-specific member banner URL.
///
/// Returns `None` if `banner_hash` is `None`.
pub fn cdn_member_banner_url(
    guild_id: &str,
    user_id: &str,
    banner_hash: Option<&str>,
    opts: &CdnOptions,
) -> Option<String> {
    let hash = banner_hash?;
    let ext = get_extension(Some(hash), opts);
    let size = append_size(opts);
    Some(format!(
        "{CDN_URL}/guilds/{guild_id}/users/{user_id}/banners/{hash}.{ext}{size}"
    ))
}

/// Build a guild icon URL.
///
/// Returns `None` if `icon_hash` is `None`.
pub fn cdn_guild_icon_url(
    guild_id: &str,
    icon_hash: Option<&str>,
    opts: &CdnOptions,
) -> Option<String> {
    let hash = icon_hash?;
    let ext = get_extension(Some(hash), opts);
    let size = append_size(opts);
    Some(format!("{CDN_URL}/icons/{guild_id}/{hash}.{ext}{size}"))
}

/// Build a guild splash URL.
///
/// Returns `None` if `splash_hash` is `None`.
pub fn cdn_guild_splash_url(
    guild_id: &str,
    splash_hash: Option<&str>,
    opts: &CdnOptions,
) -> Option<String> {
    let hash = splash_hash?;
    let ext = get_extension(Some(hash), opts);
    let size = append_size(opts);
    Some(format!(
        "{CDN_URL}/splashes/{guild_id}/{hash}.{ext}{size}"
    ))
}

/// Build a custom emoji URL.
pub fn cdn_emoji_url(emoji_id: &str, animated: bool) -> String {
    let ext = if animated { "gif" } else { "png" };
    format!("{CDN_URL}/emojis/{emoji_id}.{ext}")
}

/// Build a sticker URL.
pub fn cdn_sticker_url(sticker_id: &str, animated: bool) -> String {
    let ext = if animated { "gif" } else { "png" };
    format!("{CDN_URL}/stickers/{sticker_id}.{ext}")
}

/// Get the default avatar URL (user has no custom avatar).
///
/// Fluxer uses `fluxerstatic.com` with index = userId % 6.
pub fn cdn_default_avatar_url(user_id: &str) -> String {
    let index = user_id
        .parse::<u64>()
        .map(|n| n % 6)
        .unwrap_or(0);
    format!("{STATIC_CDN_URL}/avatars/{index}.png")
}

/// Avatar URL with fallback to default.
pub fn cdn_display_avatar_url(
    user_id: &str,
    avatar_hash: Option<&str>,
    opts: &CdnOptions,
) -> String {
    cdn_avatar_url(user_id, avatar_hash, opts).unwrap_or_else(|| cdn_default_avatar_url(user_id))
}
