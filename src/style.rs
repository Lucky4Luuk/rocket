use regex::Regex;

use tui::text::{Span, Spans};
use tui::style::{Style, Color};

lazy_static! {
    static ref PALETTE_HEADER_SELECTED: Color = Color::Rgb(42, 126, 105);
    static ref PALETTE_HEADER: Color = Color::Rgb(0,59,59);
    static ref PALETTE_LINE_NUM: Color = Color::Rgb(42, 126, 105);
    static ref PALETTE_HIGHLIGHT: Color = Color::Rgb(251, 203, 179);
    // static ref PALETTE_BACKGROUND: Color = Color::Rgb(17, 53, 44);
    static ref PALETTE_BACKGROUND: Color = Color::Rgb(32, 64, 56);
    static ref PALETTE_FOOTER: Color = Color::Rgb(0,59,59);
    static ref PALETTE_POPUP: Color = Color::Rgb(42,126,105);
    static ref PALETTE_POPUP_HEADER: Color = Color::Rgb(0,71,71);
    static ref PALETTE_BUTTON: Color = Color::Rgb(0,71,71);
    static ref PALETTE_BUTTON_SELECT: Color = Color::Rgb(191,141,124);
}

fn no_style() -> Style {
    Style::default()
}

fn rust_style(token: &str) -> Style {
    match token {
        "fn" => Style::default().fg(*PALETTE_HIGHLIGHT),
        "&" => Style::default().fg(*PALETTE_HIGHLIGHT),
        "{" | "}" | "(" | ")" => Style::default().fg(*PALETTE_HIGHLIGHT),
        _ => Style::default()
    }
}

//TODO: Don't hardcode styles you monkey
fn get_token_style(token: &str, extension: &str) -> Style {
    match extension.replace(",", "").as_ref() {
        "rs" => rust_style(token),
        _ => no_style(),
    }
}

pub fn style_line(line: String, extension: &str) -> Spans<'static> {
    let mut spans = Vec::new();
    for token in line.split(" ") {
        let style = get_token_style(token, extension);
        spans.push(Span::styled(token.to_string(), style));
        spans.push(Span::from(" "));
    }
    Spans::from(spans)
}

pub fn style_background() -> Style {
    Style::default().bg(*PALETTE_BACKGROUND)
}

lazy_static! {
    static ref LINE_NUM_RE: Regex = Regex::new(" ?[0-9]*~").expect("Failed to create `line_num_re`!");
}

pub fn editor_style(token: String) -> Span<'static> {
    if LINE_NUM_RE.is_match(&token) {
        return Span::styled(token, Style::default().fg(*PALETTE_LINE_NUM));
    }

    // let style = match token.as_ref() {
    //     "|" => Style::default().fg(Color::Red),
    //     _ => no_style(),
    // };
    // Span::styled(token, style)
    Span::from(token)
}

pub fn header_style(selected: bool) -> Style {
    let mut style = Style::default();
    if selected {
        style = style.bg(*PALETTE_HEADER_SELECTED);
    } else {
        style = style.bg(*PALETTE_HEADER);
    }

    style
}

pub fn footer_style() -> Style {
    Style::default().bg(*PALETTE_FOOTER)
}

pub fn popup_style(header: bool) -> Style {
    if header {
        Style::default().bg(*PALETTE_POPUP_HEADER)
    } else {
        Style::default().bg(*PALETTE_POPUP)
    }
}

pub fn button_style(selected: bool) -> Style {
    if selected {
        Style::default().bg(*PALETTE_BUTTON_SELECT)
    } else {
        Style::default().bg(*PALETTE_BUTTON)
    }
}
