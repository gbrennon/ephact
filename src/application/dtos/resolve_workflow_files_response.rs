use std::path::PathBuf;

/// Response DTO for the
/// [`ResolveWorkflowFilesPort`](crate::application::ports::inbound::resolve_workflow_files_port::ResolveWorkflowFilesPort)
/// inbound port.
#[derive(Debug)]
pub struct ResolveWorkflowFilesResponse {
    /// Workflow files the run executes, in execution order.
    workflow_files: Vec<PathBuf>,
}

impl ResolveWorkflowFilesResponse {
    /// Creates a new response.
    pub fn new(workflow_files: Vec<PathBuf>) -> Self {
        Self { workflow_files }
    }

    /// Workflow files the run executes, in execution order.
    pub fn workflow_files(&self) -> &[PathBuf] {
        &self.workflow_files
    }

    /// Consumes the response and returns the workflow files.
    pub fn into_workflow_files(self) -> Vec<PathBuf> {
        self.workflow_files
    }
}
