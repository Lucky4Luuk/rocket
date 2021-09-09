use tui::text::{Span, Spans};

pub fn style_line<'a>(line: String) -> Spans<'static> {
    let mut spans = Vec::new();
    for token in line.split(" ") {
        spans.push(Span::from(token.to_string()));
        spans.push(Span::from(" "));
    }
    Spans::from(spans)
}
