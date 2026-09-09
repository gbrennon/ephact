use std::path::PathBuf;

/// Response DTO for the
/// [`ListWorkflowDirectoryPort`](crate::application::ports::inbound::list_workflow_directory_port::ListWorkflowDirectoryPort)
/// inbound port.
#[derive(Debug)]
pub struct ListWorkflowDirectoryResponse {
    /// Workflow files found directly inside the directory, sorted by path.
    workflow_files: Vec<PathBuf>,
}

impl ListWorkflowDirectoryResponse {
    /// Creates a new response.
    pub fn new(workflow_files: Vec<PathBuf>) -> Self {
        Self { workflow_files }
    }

    /// Workflow files found directly inside the directory, sorted by path.
    pub fn workflow_files(&self) -> &[PathBuf] {
        &self.workflow_files
    }

    /// Consumes the response and returns the workflow files.
    pub fn into_workflow_files(self) -> Vec<PathBuf> {
        self.workflow_files
    }
}
