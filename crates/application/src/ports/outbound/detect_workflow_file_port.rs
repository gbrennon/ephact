use std::path::PathBuf;

use crate::dtos::requests::DetectWorkflowFileRequest;

/// Finds a workflow file in a repository's supported workflow directories.
pub trait DetectWorkflowFilePort: Send + Sync {
    /// Returns the first supported workflow file found for `repo_path`.
    ///
    /// # Errors
    ///
    /// Returns an error when no supported workflow directory or workflow file
    /// exists, or when the directory cannot be read.
    fn detect(
        &self,
        request: DetectWorkflowFileRequest,
    ) -> Result<PathBuf, Box<dyn std::error::Error>>;
}
