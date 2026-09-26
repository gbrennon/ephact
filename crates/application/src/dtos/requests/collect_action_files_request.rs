use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`CollectActionFilesPort`](crate::ports::inbound::collect_action_files_port::CollectActionFilesPort)
/// inbound port.
pub struct CollectActionFilesRequest {
    /// Directory whose files are collected.
    action_dir: PathBuf,
}

impl CollectActionFilesRequest {
    /// Creates a new request.
    pub fn new(action_dir: PathBuf) -> Self {
        Self { action_dir }
    }

    /// Directory whose files are collected.
    pub fn action_dir(&self) -> &Path {
        &self.action_dir
    }
}
