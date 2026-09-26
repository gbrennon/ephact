use std::path::Path;

use crate::domain::WorkflowRunConfig;

/// Request DTO for the
/// [`ResolveWorkflowFilesPort`](crate::ports::inbound::resolve_workflow_files_port::ResolveWorkflowFilesPort)
/// inbound port.
pub struct ResolveWorkflowFilesRequest<'a> {
    /// Configuration naming which workflows the run executes.
    config: &'a WorkflowRunConfig,
    /// Path to the repository the workflows are resolved in.
    repo_path: &'a Path,
}

impl<'a> ResolveWorkflowFilesRequest<'a> {
    /// Creates a new request.
    pub fn new(config: &'a WorkflowRunConfig, repo_path: &'a Path) -> Self {
        Self { config, repo_path }
    }

    /// Configuration naming which workflows the run executes.
    pub fn config(&self) -> &'a WorkflowRunConfig {
        self.config
    }

    /// Path to the repository the workflows are resolved in.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }
}
