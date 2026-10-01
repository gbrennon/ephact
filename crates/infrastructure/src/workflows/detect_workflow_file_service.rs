use std::{error::Error, path::PathBuf};

use super::workflow_directories::{WORKFLOW_DIRECTORIES, supported_workflows_display};
use crate::application::{
    dtos::requests::{DetectWorkflowFileRequest, ListWorkflowDirectoryRequest},
    ports::outbound::{DetectWorkflowFilePort, ListWorkflowDirectoryPort},
};

/// Service that detects the workflow a repository runs when the caller names
/// none, preferring the Forgejo layout over the GitHub one.
pub struct DetectWorkflowFileService {
    directory_lister: Box<dyn ListWorkflowDirectoryPort>,
}

impl DetectWorkflowFileService {
    pub fn new(directory_lister: Box<dyn ListWorkflowDirectoryPort>) -> Self {
        Self { directory_lister }
    }

    fn first_workflow_file(
        &self,
        workflows_dir: PathBuf,
        platform_dir: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        self.directory_lister
            .list(ListWorkflowDirectoryRequest::new(workflows_dir))?
            .workflow_files()
            .iter()
            .next()
            .map(|path| path.to_path_buf())
            .ok_or_else(|| format!("no workflow files found in {}/", platform_dir).into())
    }
}

impl DetectWorkflowFilePort for DetectWorkflowFileService {
    fn detect(&self, request: DetectWorkflowFileRequest) -> Result<PathBuf, Box<dyn Error>> {
        let repo_path = request.repo_path();
        let Some(platform_dir) = WORKFLOW_DIRECTORIES
            .iter()
            .find(|platform_dir| repo_path.join(platform_dir).exists())
        else {
            return Err(format!(
                "no workflows directory found ({})",
                supported_workflows_display()
            )
            .into());
        };

        self.first_workflow_file(repo_path.join(platform_dir), platform_dir)
    }
}
