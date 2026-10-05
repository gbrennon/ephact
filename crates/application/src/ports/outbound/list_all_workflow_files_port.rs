use crate::dtos::{requests::ListAllWorkflowFilesRequest, responses::ListAllWorkflowFilesResponse};

/// Lists workflow files across all supported workflow directories.
pub trait ListAllWorkflowFilesPort: Send + Sync {
    /// Returns every YAML workflow file found below the repository's supported
    /// workflow directories.
    ///
    /// # Errors
    ///
    /// Returns an error when a directory cannot be read or no workflow files
    /// are found.
    fn list(
        &self,
        request: ListAllWorkflowFilesRequest,
    ) -> Result<ListAllWorkflowFilesResponse, Box<dyn std::error::Error>>;
}
