use crate::{
    domain::errors::StepError,
    dtos::{requests::CollectActionFilesRequest, responses::CollectActionFilesResponse},
};

/// Collects an action directory into transferable file entries.
pub trait CollectActionFilesPort: Send + Sync {
    /// Recursively reads the action files, preserving relative paths and file
    /// modes while excluding the action's `.git` directory.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when the directory or one of its files cannot be
    /// read.
    fn collect(
        &self,
        request: CollectActionFilesRequest,
    ) -> Result<CollectActionFilesResponse, StepError>;
}
