#[macro_use] extern crate lazy_static;

use tui::text::Span;
use std::io;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Tabs, Paragraph, Block, Borders};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::text::{Text, Spans};

pub(crate) mod style;

pub(crate) mod editor;
use editor::Editor;

fn main() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;

    execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut editor = Editor::from_paths(vec!["test.rs", "test.txt"])?;

    'main: loop {
        terminal.draw(|f| {
            let cursor_pos = editor.cursor();
            f.set_cursor(cursor_pos.0 + 3, cursor_pos.1 + 1);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ].as_ref()
                )
                .split(f.size());

            // let file_header = Tabs::new(editor.all_filenames().cloned().map(Spans::from).collect())
            //     .divider("|");

            let mut file_header_span = Spans::from(vec![]);
            for (i, filename) in editor.all_filenames_modified().enumerate() {
                let style = style::header_style(i == editor.cur_file_idx);
                let unstyle = style::header_style(false);
                if i > 0 { file_header_span.0.push(Span::styled(" ", unstyle)); }
                file_header_span.0.push(Span::styled("[", style));
                file_header_span.0.push(Span::styled(filename, style));
                file_header_span.0.push(Span::styled("]", style));
                file_header_span.0.push(Span::styled(" ", unstyle));
            }
            let file_header = Paragraph::new(file_header_span).style(style::header_style(false));
            f.render_widget(file_header, chunks[0]);

            let cur_file_content = Paragraph::new(editor.styled_text.clone()).style(style::style_background());
            f.render_widget(cur_file_content, chunks[1]);

            let footer = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref()
                )
                .split(chunks[2]);

            let saved_text = if !editor.is_dirty() && editor.seconds_since_save().unwrap_or(9999) < 1 { "\\\\ saved!" } else { "" };
            let cursor_pos_text = Paragraph::new(Spans::from(Span::from(format!("[{}] \\\\ ({}:{}) {}", editor.path().unwrap_or(&"unsaved".to_string()), cursor_pos.0, cursor_pos.1, saved_text))))
                .style(style::footer_style());
            f.render_widget(cursor_pos_text, footer[0]);

            let rocket_text = Paragraph::new(Spans::from(Span::from("rocket // v0.0.1")))
                .alignment(Alignment::Right)
                .style(style::footer_style());
            f.render_widget(rocket_text, footer[1]);
        })?;

        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key) => {
                    if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        match key.code {
                            KeyCode::Char('q') => break 'main,
                            KeyCode::Char('r') => {}, //TODO: Command runner
                            KeyCode::Char('s') => {
                                if let Err(err) = editor.save_file() {
                                    //TODO: Handle the error
                                }
                            },
                            _ => {}
                        }
                    } else if key.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                        match key.code {
                            KeyCode::Char('u') => editor.decrement_file_idx(), //TODO: Bad shortcut
                            KeyCode::Char('i') => editor.increment_file_idx(), //TODO: Bad shortcut
                            _ => {}
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

    Ok(())
}
