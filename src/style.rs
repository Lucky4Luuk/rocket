use tui::text::{Span, Spans};
use tui::style::Style;

fn no_style() -> Style {
    Style::default()
}

fn rust_style(token: &str) -> Style {
    Style::default()
}

//TODO: Don't hardcode styles you monkey
fn get_token_style(token: &str, extension: &str) -> Style {
    match extension.replace(",", "").as_ref() {
        "rs" => rust_style(token),
        _ => no_style(),
    }
}

pub fn style_line<'a>(line: String, extension: &str) -> Spans<'static> {
    let mut spans = Vec::new();
    for token in line.split(" ") {
        let style = get_token_style(token, extension);
        spans.push(Span::styled(token.to_string(), style));
        spans.push(Span::from(" "));
    }
    Spans::from(spans)
}
