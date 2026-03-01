/// Resolve a color from hex string (`"#FF5733"`) or `[r, g, b]` array into a u32.
pub fn resolve_color(input: &str) -> Option<u32> {
    let hex = input.trim().trim_start_matches('#');
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return u32::from_str_radix(hex, 16).ok();
    }
    None
}

/// Resolve a color from an RGB array.
pub fn resolve_color_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Parse an emoji string like `<:name:id>` or `<a:name:id>`.
///
/// Returns `(name, id, animated)` or `None` if not a custom emoji.
pub fn parse_emoji(s: &str) -> Option<(String, String, bool)> {
    let s = s.trim();
    if !s.starts_with('<') || !s.ends_with('>') {
        return None;
    }
    let inner = &s[1..s.len() - 1];
    let animated = inner.starts_with("a:");
    let parts: Vec<&str> = if animated {
        inner[2..].splitn(2, ':').collect()
    } else if inner.starts_with(':') {
        inner[1..].splitn(2, ':').collect()
    } else {
        return None;
    };
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string(), animated))
    } else {
        None
    }
}

/// Format a custom emoji back into `<:name:id>` or `<a:name:id>`.
pub fn format_emoji(name: &str, id: &str, animated: bool) -> String {
    if animated {
        format!("<a:{name}:{id}>")
    } else {
        format!("<:{name}:{id}>")
    }
}

/// Parse a user mention like `<@123>` or `<@!123>`.
pub fn parse_user_mention(s: &str) -> Option<String> {
    let s = s.trim();
    if s.starts_with("<@") && s.ends_with('>') {
        let inner = &s[2..s.len() - 1];
        let id = inner.trim_start_matches('!');
        if id.chars().all(|c| c.is_ascii_digit()) && !id.is_empty() {
            return Some(id.to_string());
        }
    }
    None
}

/// Parse a role mention like `<@&123>`.
pub fn parse_role_mention(s: &str) -> Option<String> {
    let s = s.trim();
    if s.starts_with("<@&") && s.ends_with('>') {
        let id = &s[3..s.len() - 1];
        if id.chars().all(|c| c.is_ascii_digit()) && !id.is_empty() {
            return Some(id.to_string());
        }
    }
    None
}

/// Parse a prefix command from message content.
///
/// Returns `(command_name, args_string)` or `None`.
pub fn parse_prefix_command<'a>(content: &'a str, prefix: &str) -> Option<(&'a str, &'a str)> {
    let content = content.trim();
    if !content.starts_with(prefix) {
        return None;
    }
    let after = &content[prefix.len()..];
    let end = after.find(char::is_whitespace).unwrap_or(after.len());
    let cmd = &after[..end];
    let args = after[end..].trim_start();
    if cmd.is_empty() {
        return None;
    }
    Some((cmd, args))
}
