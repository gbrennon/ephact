use crate::dtos::{
    requests::ListWorkflowDirectoryRequest, responses::ListWorkflowDirectoryResponse,
};

/// Lists workflow files in one directory.
pub trait ListWorkflowDirectoryPort: Send + Sync {
    /// Returns the `.yml` and `.yaml` files directly in the requested
    /// directory, ordered by path; nested directories and other file types are
    /// ignored.
    ///
    /// # Errors
    ///
    /// Returns an error when the directory cannot be read.
    fn list(
        &self,
        request: ListWorkflowDirectoryRequest,
    ) -> Result<ListWorkflowDirectoryResponse, Box<dyn std::error::Error>>;
}
