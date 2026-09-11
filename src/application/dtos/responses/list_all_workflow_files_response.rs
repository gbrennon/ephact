use std::path::PathBuf;

/// Response DTO for the
/// [`ListAllWorkflowFilesPort`](crate::application::ports::inbound::list_all_workflow_files_port::ListAllWorkflowFilesPort)
/// inbound port.
#[derive(Debug)]
pub struct ListAllWorkflowFilesResponse {
    /// Every workflow file in the repository, `.forgejo` before `.github`.
    workflow_files: Vec<PathBuf>,
}

impl ListAllWorkflowFilesResponse {
    /// Creates a new response.
    pub fn new(workflow_files: Vec<PathBuf>) -> Self {
        Self { workflow_files }
    }

    /// Every workflow file in the repository, `.forgejo` before `.github`.
    pub fn workflow_files(&self) -> &[PathBuf] {
        &self.workflow_files
    }

    /// Consumes the response and returns the workflow files.
    pub fn into_workflow_files(self) -> Vec<PathBuf> {
        self.workflow_files
    }
}
