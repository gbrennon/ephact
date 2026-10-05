use std::path::{Path, PathBuf};

/// Request data for the outbound operation.
/// The request identifies the action directory whose files are collected.
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
