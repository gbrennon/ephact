use std::path::PathBuf;

use crate::dtos::requests::ResolveNamedWorkflowFileRequest;

/// Resolves a named workflow file in a repository.
pub trait ResolveNamedWorkflowFilePort: Send + Sync {
    /// Searches the repository root and supported workflow directories for
    /// `workflow_name`, returning the first existing path.
    ///
    /// # Errors
    ///
    /// Returns an error when the named workflow file cannot be found.
    fn resolve(
        &self,
        request: ResolveNamedWorkflowFileRequest,
    ) -> Result<PathBuf, Box<dyn std::error::Error>>;
}
