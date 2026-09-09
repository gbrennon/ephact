use std::path::Path;

use crate::domain::ActRunConfig;

/// Request DTO for the
/// [`ResolveWorkflowFilesPort`](crate::application::ports::inbound::resolve_workflow_files_port::ResolveWorkflowFilesPort)
/// inbound port.
pub struct ResolveWorkflowFilesRequest<'a> {
    /// Configuration naming which workflows the run executes.
    config: &'a ActRunConfig,
    /// Path to the repository the workflows are resolved in.
    repo_path: &'a Path,
}

impl<'a> ResolveWorkflowFilesRequest<'a> {
    /// Creates a new request.
    pub fn new(config: &'a ActRunConfig, repo_path: &'a Path) -> Self {
        Self { config, repo_path }
    }

    /// Configuration naming which workflows the run executes.
    pub fn config(&self) -> &'a ActRunConfig {
        self.config
    }

    /// Path to the repository the workflows are resolved in.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }
}
