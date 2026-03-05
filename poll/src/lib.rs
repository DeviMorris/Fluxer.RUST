use anyhow::{Result, bail};
use resvg::{tiny_skia, usvg};

const CARD_W: u32 = 800;
const PADDING: f32 = 48.0;
const HEADER_H: f32 = 148.0;
const OPTION_H: f32 = 88.0;
const FOOTER_H: f32 = 48.0;
const BAR_H: f32 = 10.0;
const BAR_MAX_W: f32 = CARD_W as f32 - PADDING * 2.0;
pub const MAX_OPTIONS: usize = 7;

const SVG_TEMPLATE: &str = include_str!("../template.svg");

const COLOR_PCT: &str = "#6ab3f3";
const COLOR_LABEL_WINNER: &str = "#ffffff";
const COLOR_LABEL_NORMAL: &str = "rgba(255,255,255,0.90)";
const COLOR_BAR_WINNER: &str = "#6ab3f3";
const COLOR_BAR_NORMAL: &str = "#3390ec";

pub struct PollOption {
    pub label: String,
    pub votes: u32,
}

pub struct PollCard {
    pub title: String,
    pub options: Vec<PollOption>,
    pub header_label: String,
    pub votes_label: String,
}

impl PollCard {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            options: Vec::new(),
            header_label: "POLL".into(),
            votes_label: "votes".into(),
        }
    }

    pub fn option(mut self, label: impl Into<String>, votes: u32) -> Self {
        self.options.push(PollOption {
            label: label.into(),
            votes,
        });
        self
    }

    pub fn header_label(mut self, label: impl Into<String>) -> Self {
        self.header_label = label.into();
        self
    }

    pub fn votes_label(mut self, label: impl Into<String>) -> Self {
        self.votes_label = label.into();
        self
    }

    pub fn render_png(&self) -> Result<Vec<u8>> {
        let svg = build_svg(self)?;
        let opt = usvg::Options::default();
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();

        let tree = usvg::Tree::from_str(&svg, &opt, &db)?;
        let w = tree.size().width() as u32;
        let h = tree.size().height() as u32;

        let mut pixmap =
            tiny_skia::Pixmap::new(w, h).ok_or_else(|| anyhow::anyhow!("pixmap alloc failed"))?;

        resvg::render(
            &tree,
            tiny_skia::Transform::identity(),
            &mut pixmap.as_mut(),
        );
        Ok(pixmap.encode_png()?)
    }
}

fn build_option(i: usize, opt: &PollOption, total: u32, max_votes: u32) -> String {
    let pct = if total == 0 {
        0.0_f32
    } else {
        opt.votes as f32 / total as f32
    };
    let pct_int = (pct * 100.0).round() as u32;
    let winner = opt.votes == max_votes && max_votes > 0;

    let label_fill = if winner {
        COLOR_LABEL_WINNER
    } else {
        COLOR_LABEL_NORMAL
    };
    let bar_fill = if winner {
        COLOR_BAR_WINNER
    } else {
        COLOR_BAR_NORMAL
    };

    let slot_top = HEADER_H + i as f32 * OPTION_H;
    let label_y = slot_top + 30.0;
    let bar_y = slot_top + 52.0;
    let bar_r = BAR_H / 2.0;
    let bar_w = (BAR_MAX_W * pct).max(if pct > 0.0 { BAR_H } else { 0.0 });

    let bar_rect = if bar_w > 0.0 {
        format!(
            r#"<rect x="{PADDING}" y="{bar_y}" width="{bar_w}" height="{BAR_H}" rx="{bar_r}" fill="{bar_fill}"/>"#,
        )
    } else {
        String::new()
    };

    format!(
        r#"<text x="{}" y="{}" font-family="Segoe UI, Inter, sans-serif" font-size="15" font-weight="700" fill="{COLOR_PCT}">{pct_int}%</text>"#,
        PADDING + 6.0,
        label_y,
    ) + &format!(
        r#"<text x="{}" y="{}" font-family="Segoe UI, Inter, sans-serif" font-size="17" font-weight="500" fill="{label_fill}">{}</text>"#,
        PADDING + 58.0,
        label_y,
        xml_escape(&opt.label),
    ) + &format!(
        r#"<rect x="{PADDING}" y="{bar_y}" width="{BAR_MAX_W}" height="{BAR_H}" rx="{bar_r}" fill="rgba(255,255,255,0.06)"/>"#,
    ) + &bar_rect
}

fn build_svg(poll: &PollCard) -> Result<String> {
    if poll.options.is_empty() || poll.options.len() > MAX_OPTIONS {
        bail!("options count must be 1–{}", MAX_OPTIONS);
    }

    let total = poll.options.iter().map(|o| o.votes).sum::<u32>();
    let max_votes = poll.options.iter().map(|o| o.votes).max().unwrap_or(0);
    let card_h = (HEADER_H + poll.options.len() as f32 * OPTION_H + FOOTER_H) as u32;
    let footer_y = card_h as f32 - 24.0;

    let options_svg: String = poll
        .options
        .iter()
        .enumerate()
        .map(|(i, opt)| build_option(i, opt, total, max_votes))
        .collect();

    Ok(SVG_TEMPLATE
        .replace("__W__", &CARD_W.to_string())
        .replace("__H__", &card_h.to_string())
        .replace("__LABEL__", &xml_escape(&poll.header_label))
        .replace("__TITLE__", &xml_escape(&poll.title))
        .replace("__SEP_X__", &(CARD_W as f32 - PADDING).to_string())
        .replace("__OPTIONS__", &options_svg)
        .replace("__FOOTER_Y__", &footer_y.to_string())
        .replace("__TOTAL_VOTES__", &total.to_string())
        .replace("__VOTES_LABEL__", &xml_escape(&poll.votes_label)))
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
