use std::iter::Iterator;
use std::time::{Instant, Duration};
use unicode_segmentation::UnicodeSegmentation;

use tui::text::Text;

use crossterm::event::{KeyEvent, KeyCode};

pub struct File {
    path: Option<String>,
    content: Vec<String>,
    cursor: (u16, u16),
    scroll: (u16, u16),

    is_dirty: bool,
    saved_time: Option<Instant>,
}

impl File {
    /// New file, has not been opened from a file
    pub fn new() -> Self {
        Self {
            path: None,
            content: Vec::new(),
            cursor: (0, 0),
            scroll: (0, 0),

            is_dirty: false,
            saved_time: None,
        }
    }

    /// Open a file from a path
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: Some(path.to_string()),
            content: std::fs::read_to_string(path)?.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
            cursor: (0, 0),
            scroll: (0, 0),

            is_dirty: false,
            saved_time: None,
        })
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(path) = &self.path {
            let content: String = self.content.iter().map(|s| {
                let mut s = String::from(s);
                s.push('\n');
                s
            }).collect();
            std::fs::write(path, content)?;
            self.is_dirty = false;
            self.saved_time = Some(Instant::now());
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "No path!"))
        }
    }

    pub fn path(&self) -> Option<&String> {
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

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn saved_time(&self) -> Option<Instant> {
        self.saved_time
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

    pub fn add_line(&mut self) {
        if self.cursor.0 >= self.line_length() {
            if self.cursor.1 >= self.line_count()-1 {
                self.content.push(String::new());
            } else {
                self.content.insert(self.cursor.1 as usize + 1, String::new());
            }
        } else {
            //Not at the end of a line, so we have to split the current line and move it the right side down
            let left: String = self.content[self.cursor.1 as usize][..].graphemes(true).take(self.cursor.0 as usize).collect();
            let right: String = self.content[self.cursor.1 as usize][..].graphemes(true).skip(self.cursor.0 as usize).collect();
            self.content[self.cursor.1 as usize] = left;
            self.content.insert(self.cursor.1 as usize + 1, right);
        }
        self.move_cursor(0, 1);
        self.cursor.0 = 0;
        self.is_dirty = true;
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
        self.move_cursor(1, 0);
        self.is_dirty = true;
    }

    pub fn remove_character(&mut self) {
        if self.cursor.0 == 0 {
            if self.cursor.1 > 0 {
                let cur = self.content[self.cursor.1 as usize].clone();
                let next_x = self.content[self.cursor.1 as usize - 1].len() as u16;
                self.content[self.cursor.1 as usize - 1].push_str(&cur);
                self.content.remove(self.cursor.1 as usize);
                // self.move_cursor(0,-1);
                self.cursor.1 -= 1;
                self.cursor.0 = next_x;
                self.is_dirty = true;
            }
        } else if self.cursor.0 > 0 {
            //Not at the start of a line, so we have to remove characters
            let mut left: String = self.content[self.cursor.1 as usize][..].graphemes(true).take(self.cursor.0 as usize - 1).collect();
            let right: String = self.content[self.cursor.1 as usize][..].graphemes(true).skip(self.cursor.0 as usize).collect();
            left.push_str(&right);
            self.content[self.cursor.1 as usize] = left;
            self.move_cursor(-1, 0);
            self.is_dirty = true;
        }
    }

    pub fn delete_character(&mut self) {
        if self.cursor.0 < self.line_length() {
            self.move_cursor(1, 0);
            self.remove_character();
            // self.move_cursor(-1, 0);
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

    pub fn save_file(&mut self) -> Result<(), std::io::Error> {
        self.open_files[self.cur_file_idx].save()
    }

    fn update_styled_text(&mut self) {
        let mut content_spans = Vec::new();
        let lines = self.content();
        let max_nums = lines.len().to_string().chars().count();
        for (i, line) in lines.into_iter().enumerate() {
            let line_num = format!("{:width$}~ ", i, width = max_nums);
            let line = format!("{}", line.replace('\t', "    "));
            let styled_line_num = crate::style::editor_style(line_num);
            let mut styled_line = crate::style::style_line(line, self.extension());
            styled_line.0.insert(0, styled_line_num);
            content_spans.push(styled_line);
        }
        self.styled_text = Text::from(content_spans);
    }

    pub fn all_filenames(&self) -> impl Iterator<Item = &str> {
        use std::path::Path;
        use std::ffi::OsStr;
        self.all_paths().map(Path::new).map(|p| p.file_name().unwrap_or(OsStr::new("unsaved"))).map(|s| s.to_str().expect("Filenames with non-unicode characters are not supported!"))
    }

    pub fn all_filenames_modified(&self) -> impl Iterator<Item = String> + '_ {
        self.all_filenames().enumerate().map(move |(i, s)| {
            if self.open_files[i].is_dirty() {
                format!("*{}", s)
            } else {
                s.to_string()
            }
        })
    }

    pub fn all_paths(&self) -> impl Iterator<Item = &String> {
        self.open_files.iter().filter_map(File::path)
    }

    pub fn path(&self) -> Option<&String> {
        self.open_files[self.cur_file_idx].path()
    }

    pub fn extension(&self) -> &str {
        if let Some(filename) = self.path() {
            std::path::Path::new(filename).extension().unwrap_or(std::ffi::OsStr::new("")).to_str().expect("Extension contains non-Unicode characters!")
        } else {
            ""
        }
    }

    pub fn content(&self) -> &Vec<String> {
        self.open_files[self.cur_file_idx].content()
    }

    pub fn is_dirty(&self) -> bool {
        self.open_files[self.cur_file_idx].is_dirty()
    }

    pub fn seconds_since_save(&self) -> Option<u64> {
        Some(self.open_files[self.cur_file_idx].saved_time()?.elapsed().as_secs())
    }

    pub fn cursor(&self) -> (u16, u16) {
        self.open_files[self.cur_file_idx].cursor_unscrolled()
    }

    pub fn increment_file_idx(&mut self) {
        self.cur_file_idx += 1;
        if self.cur_file_idx >= self.open_files.len() {
            self.cur_file_idx = 0;
        }
        self.update_styled_text();
    }

    pub fn decrement_file_idx(&mut self) {
        if self.cur_file_idx > 0 {
            self.cur_file_idx -= 1;
        } else {
            self.cur_file_idx = self.open_files.len() - 1;
        }
        self.update_styled_text();
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

            KeyCode::Enter => {
                self.open_files[self.cur_file_idx].add_line();
                // self.move_cursor(0, 1);
                self.update_styled_text();
            },

            KeyCode::Backspace => {
                self.open_files[self.cur_file_idx].remove_character();
                self.update_styled_text();
            },

            KeyCode::Delete => {
                self.open_files[self.cur_file_idx].delete_character();
                self.update_styled_text();
            },

            KeyCode::Tab => {
                for _ in 0..4 {
                    self.open_files[self.cur_file_idx].add_character(' ');
                }
                // self.open_files[self.cur_file_idx].add_character('\t');
                self.update_styled_text();
            },

            KeyCode::Char(c) => {
                //TODO: Check modifiers
                self.open_files[self.cur_file_idx].add_character(c);
                // self.move_cursor(1, 0);
                self.update_styled_text();
            },

            _ => {},
        }
    }
}
