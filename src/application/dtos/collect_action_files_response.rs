use crate::application::dtos::FileEntry;

/// Response DTO for the
/// [`CollectActionFilesPort`](crate::application::ports::inbound::collect_action_files_port::CollectActionFilesPort)
/// inbound port.
#[derive(Debug)]
pub struct CollectActionFilesResponse {
    /// Files making up the action, with paths relative to its directory.
    pub files: Vec<FileEntry>,
}

impl CollectActionFilesResponse {
    /// Creates a new response.
    pub fn new(files: Vec<FileEntry>) -> Self {
        Self { files }
    }

    /// Files making up the action, with paths relative to its directory.
    pub fn files(&self) -> &[FileEntry] {
        &self.files
    }

    /// Consumes the response and returns the collected files.
    pub fn into_files(self) -> Vec<FileEntry> {
        self.files
    }
}
