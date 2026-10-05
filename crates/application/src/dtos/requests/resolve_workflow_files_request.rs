use std::path::{Path, PathBuf};

use crate::domain::WorkflowRunConfig;

/// Request data for the outbound operation.
/// The request supplies run selection settings and the repository path.
pub struct ResolveWorkflowFilesRequest {
    /// Configuration naming which workflows the run executes.
    config: WorkflowRunConfig,
    /// Path to the repository the workflows are resolved in.
    repo_path: PathBuf,
}

impl ResolveWorkflowFilesRequest {
    /// Creates a new owned request from the supplied inputs.
    pub fn new(config: &WorkflowRunConfig, repo_path: &Path) -> Self {
        Self {
            config: config.clone(),
            repo_path: repo_path.to_path_buf(),
        }
    }

    /// Configuration naming which workflows the run executes.
    pub fn config(&self) -> &WorkflowRunConfig {
        &self.config
    }

    /// Path to the repository the workflows are resolved in.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
