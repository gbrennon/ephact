use crate::dtos::{requests::ResolveWorkflowFilesRequest, responses::ResolveWorkflowFilesResponse};

/// Selects workflow files according to a run configuration.
pub trait ResolveWorkflowFilesPort: Send + Sync {
    /// Lists all workflows when requested, resolves a named workflow when
    /// configured, or detects the first workflow otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected workflow files cannot be resolved.
    fn resolve(
        &self,
        request: ResolveWorkflowFilesRequest,
    ) -> Result<ResolveWorkflowFilesResponse, Box<dyn std::error::Error>>;
}
