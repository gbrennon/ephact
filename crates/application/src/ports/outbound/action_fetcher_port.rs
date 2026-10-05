use std::path::PathBuf;

use crate::domain::{errors::ActionError, value_objects::RemoteActionReference};

/// Fetches remote action sources for local execution.
pub trait ActionFetcherPort: Send + Sync {
    /// Fetches the repository and revision identified by
    /// [`RemoteActionReference`] and returns its local checkout directory.
    ///
    /// Implementations may reuse a local cache for an already fetched
    /// reference.
    ///
    /// # Errors
    ///
    /// Returns [`ActionError`] when the source cannot be fetched or cached.
    fn fetch(&self, reference: &RemoteActionReference) -> Result<PathBuf, ActionError>;
    ///
    /// Creates a boxed clone of this fetcher.
    fn clone_box(&self) -> Box<dyn ActionFetcherPort>;
}

impl Clone for Box<dyn ActionFetcherPort> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
