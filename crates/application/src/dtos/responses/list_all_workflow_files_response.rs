use std::path::PathBuf;

/// Response data returned by the outbound operation.
#[derive(Debug)]
pub struct ListAllWorkflowFilesResponse {
    /// Every workflow file discovered in the repository.
    workflow_files: Vec<PathBuf>,
}

impl ListAllWorkflowFilesResponse {
    /// Creates a new response.
    pub fn new(workflow_files: Vec<PathBuf>) -> Self {
        Self { workflow_files }
    }

    /// Every workflow file discovered in the repository.
    pub fn workflow_files(&self) -> &[PathBuf] {
        &self.workflow_files
    }

    /// Consumes the response and returns the workflow files.
    pub fn into_workflow_files(self) -> Vec<PathBuf> {
        self.workflow_files
    }
}
