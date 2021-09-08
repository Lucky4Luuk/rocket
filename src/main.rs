use std::io;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Tabs, Paragraph, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Text, Spans};

mod editor;
use editor::Editor;

fn main() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;

    execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut editor = Editor::from_paths(vec!["test.txt"])?;

    'main: loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(f.size());

            //TODO: Styling (and highlight open file based on index)
            let file_header = Tabs::new(editor.all_filenames().cloned().map(Spans::from).collect())
                .divider("|");
            f.render_widget(file_header, chunks[0]);

            let mut content_spans = Vec::new();
            let lines: Vec<&str> = editor.content().lines().collect();
            let max_nums = lines.len().to_string().chars().count();
            for (i, line) in lines.into_iter().enumerate() {
                content_spans.push(Spans::from(format!("{:width$}~ {}", i, line, width = max_nums)));
            }
            let cur_file_content = Paragraph::new(content_spans);
            f.render_widget(cur_file_content, chunks[1]);
        })?;

        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key) => {
                    if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        if key.code == KeyCode::Char('q') {
                            break 'main;
                        }
                    } else {
                        editor.handle_key(key);
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
