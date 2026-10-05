use std::path::PathBuf;

use crate::{domain::errors::ActionError, dtos::requests::FetchRemoteActionRequest};

/// Resolves a remote action reference to a local directory.
pub trait FetchRemoteActionPort: Send + Sync {
    /// Fetches the referenced action and appends its optional subdirectory to
    /// the returned checkout path.
    ///
    /// # Errors
    ///
    /// Returns [`ActionError`] when the remote action cannot be fetched.
    fn fetch(&self, request: FetchRemoteActionRequest) -> Result<PathBuf, ActionError>;
}
