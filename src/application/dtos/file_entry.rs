/// A file entry for copy operations.
#[derive(Debug, Clone)]
pub struct FileEntry {
    path: String,
    content: Vec<u8>,
    mode: u32,
}

impl FileEntry {
    pub fn new(path: impl Into<String>, content: Vec<u8>, mode: u32) -> Self {
        Self {
            path: path.into(),
            content,
            mode,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn mode(&self) -> u32 {
        self.mode
    }

    pub fn into_parts(self) -> (String, Vec<u8>, u32) {
        (self.path, self.content, self.mode)
    }
}
