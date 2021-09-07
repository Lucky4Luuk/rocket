pub struct Editor {
    pub folder_path: String,

    pub current_filename: Option<String>,
    pub content: Option<String>,
}

impl Editor {
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let content = String::from("Open a file to start working!");
        Ok(Self {
            folder_path: path.to_string(),

            current_filename: None,
            content: None,
        })
    }

    pub fn filename(&self) -> Result<String, ()> {
        match &self.current_filename {
            Some(f) => Ok(f.clone()),
            None => Err(())
        }
    }
}
