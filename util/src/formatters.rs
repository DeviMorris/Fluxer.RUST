/// Truncate a string to `max_len`, appending `…` if truncated.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    let mut result: String = s.chars().take(max_len.saturating_sub(1)).collect();
    result.push('…');
    result
}

/// Escape Discord/Fluxer markdown characters.
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

/// Format an RGB color number as hex string (e.g. `0xFF5733` → `"#FF5733"`).
pub fn format_color(color: u32) -> String {
    format!("#{:06X}", color & 0xFFFFFF)
}

/// Format a Unix timestamp (seconds) as a Discord-style timestamp tag.
///
/// `style` is one of: `t`, `T`, `d`, `D`, `f`, `F`, `R`.
pub fn format_timestamp(unix_secs: u64, style: Option<char>) -> String {
    match style {
        Some(s) => format!("<t:{unix_secs}:{s}>"),
        None => format!("<t:{unix_secs}>"),
    }
}
