use std::io;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

mod editor;
use editor::Editor;

fn main() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;

    execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let editor = Editor::from_path("test.txt")?;

    'main: loop {
        let current_filename = editor.filename().unwrap_or("empty".to_string());

        terminal.draw(|f| {
            let full_border = Block::default().borders(Borders::ALL);
            f.render_widget(full_border, f.size());
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(2),
                        Constraint::Percentage(96),
                        Constraint::Percentage(2)
                    ].as_ref()
                )
                .split(f.size());
            let header = Block::default()
                .title("Rocket editor [v0.0.1]")
                .borders(Borders::ALL);
            f.render_widget(header, chunks[0]);
            let mid_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(13),
                        Constraint::Percentage(87)
                    ].as_ref()
                )
                .split(chunks[1]);
            let file_browser = Block::default()
                .title("Files")
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM);
            f.render_widget(file_browser, mid_chunks[0]);
            let file = Block::default()
                .title(current_filename)
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);
            f.render_widget(file, mid_chunks[1]);
            let footer = Block::default()
                .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);
            f.render_widget(footer, chunks[2]);
        })?;

        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Esc {
                        break 'main;
                    }
                },
                _ => {},
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(io::stdout(), crossterm::style::ResetColor, crossterm::cursor::Show, crossterm::terminal::LeaveAlternateScreen)?;

    println!("");

    Ok(())
}
