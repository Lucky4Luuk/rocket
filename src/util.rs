use tui::layout::{Layout, Direction, Constraint, Rect};

pub fn centered_rect_perc(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn centered_rect_set(x: u16, y: u16, r: Rect) -> Rect {
    let percent_x = ((x as f32) / (r.width as f32) * 100f32) as u16;
    let percent_y = ((y as f32) / (r.height as f32) * 100f32) as u16;
    centered_rect_perc(percent_x, percent_y, r)
}
