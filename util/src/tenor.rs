/// Returns `true` if `url` looks like a Tenor link.
pub fn is_tenor_url(url: &str) -> bool {
    url.starts_with("https://tenor.com/view/")
        || url.starts_with("https://tenor.com/embed/")
        || url.starts_with("https://media.tenor.com/")
}

/// Extracts Tenor GIF id from share/embed URLs.
pub fn extract_tenor_id(url: &str) -> Option<String> {
    let trimmed = url.trim_end_matches('/');

    if let Some(path) = trimmed.strip_prefix("https://tenor.com/view/") {
        return path.rsplit('-').next().and_then(|s| {
            if s.chars().all(|c| c.is_ascii_digit()) && !s.is_empty() {
                Some(s.to_string())
            } else {
                None
            }
        });
    }

    if let Some(id) = trimmed.strip_prefix("https://tenor.com/embed/")
        && !id.is_empty()
        && id.chars().all(|c| c.is_ascii_digit())
    {
        return Some(id.to_string());
    }

    None
}

/// Builds a best-effort direct GIF URL for a Tenor id.
pub fn tenor_media_url(gif_id: &str) -> String {
    format!("https://media.tenor.com/images/{gif_id}/tenor.gif")
}

/// Resolves a Tenor URL to a best-effort direct GIF URL.
pub fn resolve_tenor_to_image_url(url: &str) -> Option<String> {
    extract_tenor_id(url).map(|id| tenor_media_url(&id))
}
