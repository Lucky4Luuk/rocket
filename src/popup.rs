use unicode_segmentation::UnicodeSegmentation;

use crossterm::event::{KeyEvent, KeyCode};

pub enum PopupButton {
    Ok,
    /// Quirky version of Ok
    IGotIt,
    Cancel,
}

impl PopupButton {
    pub fn get_text(&self) -> &str {
        match self {
            Self::Ok => "okay",
            Self::IGotIt => "i got it!",
            Self::Cancel => "cancel",
        }
    }
}

#[non_exhaustive]
pub enum PopupKind {
    Help,
    Dialogue(String),
    SaveFile(String),
    LoadFile(String),
    IOError(String),
}

impl PopupKind {
    pub fn get_buttons(&self) -> Vec<PopupButton> {
        match self {
            Self::Help => vec![PopupButton::IGotIt],
            Self::Dialogue(_) => vec![PopupButton::Ok],
            Self::SaveFile(_) => vec![PopupButton::Cancel, PopupButton::Ok],
            Self::LoadFile(_) => vec![PopupButton::Cancel, PopupButton::Ok],
            Self::IOError(_) => vec![PopupButton::Ok],
            _ => vec![],
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Self::Help => "help menu",
            Self::Dialogue(_) => "dialogue",
            Self::SaveFile(_) => "save file",
            Self::LoadFile(_) => "load file",
            Self::IOError(_) => "io error",
        }
    }

    pub fn content(&self) -> String {
        match self {
            Self::Help => {
"ctrl + h // this menu   |   alt + u  // next file
ctrl + q // quit        |   alt + i  // next file
ctrl + s // save        |   ctrl + w // close file
ctrl + t // save as     |
ctrl + o // open file   |".to_string()
            }
            Self::Dialogue(s) => s.clone(),
            Self::SaveFile(s) => format!("path >> {}", &s),
            Self::LoadFile(s) => format!("path >> {}", &s),
            Self::IOError(s) => s.clone(),
        }
    }
}

pub struct Popup {
    kind: PopupKind,
    pub buttons: Vec<PopupButton>,
    pub button_idx: usize,
}

impl Popup {
    pub fn from_kind(kind: PopupKind) -> Self {
        let buttons = kind.get_buttons();
        Self {
            kind: kind,
            buttons: buttons,
            button_idx: 0,
        }
    }

    pub fn title(&self) -> &str {
        self.kind.title()
    }

    pub fn content(&self) -> String {
        self.kind.content()
    }

    fn handle_enter(&mut self, editor: &mut crate::editor::Editor) -> bool {
        match &self.kind {
            PopupKind::Help | PopupKind::Dialogue(_) | PopupKind::IOError(_) => return true, //Only has an Ok button, and needs no logic. Just close it
            PopupKind::SaveFile(path) => {
                match self.buttons[self.button_idx] {
                    PopupButton::Ok => {
                        if let Err(err) = editor.save_file_to_path(path.to_string()) {
                            *self = Popup::from_kind(PopupKind::IOError(err.to_string()));
                            return false;
                        }
                        return true;
                    },
                    _ => return true,
                };
            },
            PopupKind::LoadFile(path) => {
                match self.buttons[self.button_idx] {
                    PopupButton::Ok => {
                        if let Err(err) = editor.load_file_from_path(path.to_string()) {
                            *self = Popup::from_kind(PopupKind::IOError(err.to_string()));
                            return false;
                        }
                        return true;
                    },
                    _ => return true,
                };
            },
            _ => {},
        }
        false
    }

    pub fn handle_key(&mut self, key: KeyEvent, editor: &mut crate::editor::Editor) -> bool {
        match key.code {
            KeyCode::Left => if self.button_idx == 0 { self.button_idx = self.buttons.len()-1 } else { self.button_idx -= 1 },
            KeyCode::Right => self.button_idx = (self.button_idx + 1) % self.buttons.len(),
            KeyCode::Enter => {
                return self.handle_enter(editor);
            },
            KeyCode::Char(c) => {
                match &mut self.kind {
                    PopupKind::SaveFile(path) | PopupKind::LoadFile(path) => {
                        path.push(c);
                    },
                    _ => {},
                }
            },
            KeyCode::Backspace => {
                match &mut self.kind {
                    PopupKind::SaveFile(path) | PopupKind::LoadFile(path) => {
                        let new_path: String = path[..].graphemes(true).take(path.len()-1).collect();
                        *path = new_path;
                    },
                    _ => {},
                }
            }
            _ => {},
        }
        false
    }
}
