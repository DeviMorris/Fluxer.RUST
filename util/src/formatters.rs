/// Truncates `s` to at most `max_len` chars, appending `…` if truncated.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    let mut result: String = s.chars().take(max_len.saturating_sub(1)).collect();
    result.push('\u{2026}');
    result
}

/// Escapes common markdown control characters.
pub fn escape_markdown(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if matches!(ch, '*' | '_' | '~' | '`' | '|' | '>' | '#') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// Formats an RGB integer color to `#RRGGBB`.
pub fn format_color(color: u32) -> String {
    format!("#{:06X}", color & 0xFFFFFF)
}

/// Formats a Discord-style timestamp tag.
pub fn format_timestamp(unix_secs: u64, style: Option<char>) -> String {
    match style {
        Some(s) => format!("<t:{unix_secs}:{s}>"),
        None => format!("<t:{unix_secs}>"),
    }
}
