use crate::domain::entities::FileEntry;

/// A file entry for copy operations.
#[derive(Debug, Clone)]
pub struct FileEntryResponse {
    path: String,
    content: Vec<u8>,
    mode: u32,
}

impl FileEntryResponse {
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

impl From<FileEntryResponse> for FileEntry {
    fn from(value: FileEntryResponse) -> Self {
        let (path, content, mode) = value.into_parts();
        Self::new(path, content, mode)
    }
}

impl From<FileEntry> for FileEntryResponse {
    fn from(value: FileEntry) -> Self {
        let (path, content, mode) = value.into_parts();
        Self::new(path, content, mode)
    }
}
