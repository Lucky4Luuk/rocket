use std::iter::Iterator;

use crossterm::event::KeyEvent;

pub struct File {
    path: Option<String>,
    content: String,
    cursor: (usize, usize),
}

impl File {
    /// New file, has not been opened from a file
    pub fn new() -> Self {
        Self {
            path: None,
            content: String::new(),
            cursor: (0, 0),
        }
    }

    /// Open a file from a path
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            path: Some(path.to_string()),
            content: std::fs::read_to_string(path)?
        })
    }

    pub fn filename(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }
}

pub struct Editor {
    pub open_files: Vec<File>,
    pub cur_file_idx: usize,

    pub cursor: (usize, usize),
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
        Ok(Self {
            open_files: files,
            cur_file_idx: 0,
        })
    }

    pub fn all_filenames(&self) -> impl Iterator<Item = &String> {
        self.open_files.iter().filter_map(File::filename)
    }

    pub fn filename(&self) -> Option<&String> {
        self.open_files[self.cur_file_idx].filename()
    }

    pub fn content(&self) -> &str {
        self.open_files[self.cur_file_idx].content()
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.open_files[self.cur_file_idx].cursor()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {

    }
}
