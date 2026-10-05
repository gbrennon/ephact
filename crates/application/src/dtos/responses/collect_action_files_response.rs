use crate::dtos::responses::FileEntryResponse;

/// Response data returned by the outbound operation.
#[derive(Debug)]
pub struct CollectActionFilesResponse {
    /// Files making up the action, with paths relative to its directory.
    files: Vec<FileEntryResponse>,
}

impl CollectActionFilesResponse {
    /// Creates a new response.
    pub fn new(files: Vec<FileEntryResponse>) -> Self {
        Self { files }
    }

    /// Files making up the action, with paths relative to its directory.
    pub fn files(&self) -> &[FileEntryResponse] {
        &self.files
    }

    /// Consumes the response and returns the collected files.
    pub fn into_files(self) -> Vec<FileEntryResponse> {
        self.files
    }
}
