#[macro_use] extern crate lazy_static;

use std::sync::Mutex;
use std::io;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;

use tui::Terminal;
use tui::text::Span;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Tabs, Paragraph, Block, Borders};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::text::{Text, Spans};

pub(crate) mod style;

pub(crate) mod editor;
use editor::Editor;

pub(crate) mod popup;
use popup::{Popup, PopupKind};

pub(crate) mod util;

lazy_static! {
    static ref POPUP_STACK: Mutex<Vec<Popup>> = Mutex::new(Vec::new());
}

fn main() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;

    execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut editor = Editor::from_paths(vec!["test.rs", "test.txt", "path_test/test.c"])?;

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

            {
                let stack = POPUP_STACK.lock().expect("Failed to get lock on POPUP_STACK!");
                if stack.len() > 0 {
                    let popup = &stack.last().unwrap();

                    let h = 3 + popup.content().lines().count().max(1);

                    let popup_rect = util::centered_rect_set(50, h as u16, f.size());
                    let popup_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints(
                            [
                                Constraint::Length(1),
                                Constraint::Min(1),
                                Constraint::Length(1), //For the buttons
                            ].as_ref()
                        )
                        .split(popup_rect);

                    let popup_header = Paragraph::new(popup.title()).style(style::popup_style(true));
                    f.render_widget(popup_header, popup_layout[0]);
                    let popup_content = Paragraph::new(popup.content()).style(style::popup_style(false));
                    f.render_widget(popup_content, popup_layout[1]);

                    let button_spacing = 5;
                    let button_perc = ((1f32 / popup.buttons.len() as f32) * 100f32) as u16 - ((1f32 / (popup.buttons.len()-1) as f32) * 2.5f32) as u16; //Adds 5 percent spacing to the calculation
                    let button_constraints: Vec<Constraint> = (0..popup.buttons.len()*2-1).enumerate().map(|(i, _)| if i%2==0 { Constraint::Percentage(button_perc) } else { Constraint::Percentage(button_spacing) }).collect();

                    f.render_widget(Block::default().style(style::popup_style(false)), popup_layout[2]);

                    let popup_button_layout = Layout::default()
                        .direction(Direction::Horizontal)
                        .horizontal_margin(button_spacing)
                        .constraints(button_constraints)
                        .split(popup_layout[2]);

                    for (i, button) in popup.buttons.iter().enumerate() {
                        let button_widget = Paragraph::new(button.get_text()).style(style::button_style(i == popup.button_idx)).alignment(Alignment::Center);
                        f.render_widget(button_widget, popup_button_layout[i*2]);
                    }
                }
            }
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
                                    let mut stack = POPUP_STACK.lock().expect("Failed to get lock on POPUP_STACK!");
                                    stack.push(Popup::from_kind(PopupKind::IOError(err.to_string())));
                                }
                            },
                            KeyCode::Char('h') => {
                                let mut stack = POPUP_STACK.lock().expect("Failed to get lock on POPUP_STACK!");
                                stack.push(Popup::from_kind(PopupKind::Help));
                            },
                            KeyCode::Char('t') => {
                                let mut stack = POPUP_STACK.lock().expect("Failed to get lock on POPUP_STACK!");
                                stack.push(Popup::from_kind(PopupKind::SaveFile(String::new())));
                            },
                            _ => {}
                        }
                    } else if key.modifiers.contains(crossterm::event::KeyModifiers::ALT) {
                        match key.code {
                            KeyCode::Char('u') => editor.decrement_file_idx(), //TODO: Bad shortcut
                            KeyCode::Char('i') => editor.increment_file_idx(), //TODO: Bad shortcut
                            _ => {}
                        }
                    } else if POPUP_STACK.lock().expect("Failed to get lock on POPUP_STACK!").len() > 0 {
                        let mut stack = POPUP_STACK.lock().expect("Failed to get lock on POPUP_STACK!");
                        if stack.len() > 0 {
                            if stack.last_mut().unwrap().handle_key(key, &mut editor) {
                                stack.pop();
                            }
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
