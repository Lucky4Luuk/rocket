use crossterm::event::{KeyEvent, KeyCode};

pub enum PopupButton {
    Ok,
    Cancel,
}

impl PopupButton {
    pub fn get_text(&self) -> &str {
        match self {
            Self::Ok => "okay",
            Self::Cancel => "cancel",
        }
    }
}

#[non_exhaustive]
pub enum PopupKind {
    Dialogue(String),
    SaveFile(String),
    LoadFile(String),
    IOError(String),
}

impl PopupKind {
    pub fn get_buttons(&self) -> Vec<PopupButton> {
        match self {
            Self::Dialogue(_) => vec![PopupButton::Cancel, PopupButton::Ok],
            Self::SaveFile(_) => vec![PopupButton::Cancel, PopupButton::Ok],
            Self::LoadFile(_) => vec![PopupButton::Cancel, PopupButton::Cancel],
            Self::IOError(_) => vec![PopupButton::Ok],
            _ => vec![],
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Self::Dialogue(_) => "dialogue",
            Self::SaveFile(_) => "save file",
            Self::LoadFile(_) => "load file",
            Self::IOError(_) => "io error",
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Self::Dialogue(s) => &s,
            Self::SaveFile(s) => &s,
            Self::LoadFile(s) => &s,
            Self::IOError(s) => &s,
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

    pub fn content(&self) -> &str {
        self.kind.content()
    }

    fn handle_enter(&mut self, editor: &mut crate::editor::Editor) -> bool {
        match self.kind {
            PopupKind::Dialogue(_) => return true, //Only has an Ok button, and needs no logic. Just close it
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
            _ => {},
        }
        false
    }
}
