use std::iter::Iterator;
use unicode_segmentation::UnicodeSegmentation;

use tui::text::Text;

use crossterm::event::{KeyEvent, KeyCode};

pub struct File {
    path: Option<String>,
    content: Vec<String>,
    cursor: (u16, u16),
    scroll: (u16, u16),
}

impl File {
    /// New file, has not been opened from a file
    pub fn new() -> Self {
        Self {
            path: None,
            content: Vec::new(),
            cursor: (0, 0),
            scroll: (0, 0),
        }
    }

    /// Open a file from a path
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: Some(path.to_string()),
            content: std::fs::read_to_string(path)?.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
            cursor: (0, 0),
            scroll: (0, 0),
        })
    }

    pub fn filename(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn cursor(&self) -> (u16, u16) {
        self.cursor
    }

    pub fn cursor_unscrolled(&self) -> (u16, u16) {
        (self.cursor.0 - self.scroll.1, self.cursor.1 - self.scroll.1)
    }

    fn line_length(&self) -> u16 {
        self.content[self.cursor.1 as usize].len() as u16
    }

    fn line_count(&self) -> u16 {
        self.content.len() as u16
    }

    pub fn move_cursor(&mut self, dx: i16, dy: i16) {
        if dx < 0  {
            if self.cursor.0 > 0 {
                self.cursor.0 -= dx.abs() as u16;
            }
        } else {
            if self.cursor.0 < self.line_length() {
                self.cursor.0 += dx as u16;
            }
        }

        if dy < 0 {
            if self.cursor.1 > 0 {
                self.cursor.1 -= dy.abs() as u16;
                self.cursor.0 = self.cursor.0.min(self.line_length());
            }
        } else {
            if self.cursor.1 < self.line_count() - 1 {
                self.cursor.1 += dy as u16;
                self.cursor.0 = self.cursor.0.min(self.line_length());
            }
        }
    }

    pub fn add_character(&mut self, c: char) {
        if self.cursor.0 >= self.line_length() {
            self.content[self.cursor.1 as usize].push(c);
        } else {
            //We are not at the end of a line, so we have to split, push and merge
            let mut left: String = self.content[self.cursor.1 as usize][..].graphemes(true).take(self.cursor.0 as usize).collect();
            let right: String = self.content[self.cursor.1 as usize][..].graphemes(true).skip(self.cursor.0 as usize).collect();
            left.push(c);
            left.push_str(&right);
            self.content[self.cursor.1 as usize] = left;
        }
    }
}

pub struct Editor {
    pub open_files: Vec<File>,
    pub cur_file_idx: usize,

    pub styled_text: Text<'static>,
}

impl Editor {
    pub fn from_paths(paths: Vec<&str>) -> Result<Self, std::io::Error> {
        let files = if paths.is_empty() {
            vec![File::new()]
        } else {
            // paths.iter().map(|s| File::from_path(*s)).collect()
            let mut tmp = Vec::new();
            for path in paths {
                tmp.push(File::from_path(path)?);
            }
            tmp
        };
        let mut obj = Self {
            open_files: files,
            cur_file_idx: 0,

            styled_text: Text::default(),
        };
        obj.update_styled_text();
        Ok(obj)
    }

    fn update_styled_text(&mut self) {
        let mut content_spans = Vec::new();
        let lines = self.content();
        let max_nums = lines.len().to_string().chars().count();
        for (i, line) in lines.into_iter().enumerate() {
            let line = format!("{:width$}~ {}", i, line, width = max_nums);
            let styled_line = crate::style::style_line(line);
            content_spans.push(styled_line);
        }
        self.styled_text = Text::from(content_spans);
    }

    pub fn all_filenames(&self) -> impl Iterator<Item = &String> {
        self.open_files.iter().filter_map(File::filename)
    }

    pub fn filename(&self) -> Option<&String> {
        self.open_files[self.cur_file_idx].filename()
    }

    pub fn content(&self) -> &Vec<String> {
        self.open_files[self.cur_file_idx].content()
    }

    pub fn cursor(&self) -> (u16, u16) {
        self.open_files[self.cur_file_idx].cursor_unscrolled()
    }

    fn move_cursor(&mut self, dx: i16, dy: i16) {
        self.open_files[self.cur_file_idx].move_cursor(dx,dy);
        // self.update_styled_text();
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.move_cursor(-1, 0),
            KeyCode::Right => self.move_cursor(1, 0),
            KeyCode::Up => self.move_cursor(0,-1),
            KeyCode::Down => self.move_cursor(0, 1),

            KeyCode::Char(c) => {
                //TODO: Check modifiers
                self.open_files[self.cur_file_idx].add_character(c);
                self.move_cursor(1, 0);
                self.update_styled_text();
            },

            _ => {},
        }
    }
}
