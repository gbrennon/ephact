use std::{error::Error, path::PathBuf};

use super::{
    detect_workflow_file_port::DetectWorkflowFilePort,
    list_workflow_directory_port::ListWorkflowDirectoryPort,
    workflow_directories::{WORKFLOW_DIRECTORIES, supported_workflows_display},
};
use crate::application::dtos::requests::{DetectWorkflowFileRequest, ListWorkflowDirectoryRequest};

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
            .execute(ListWorkflowDirectoryRequest::new(workflows_dir))?
            .workflow_files()
            .iter()
            .next()
            .map(|path| path.to_path_buf())
            .ok_or_else(|| format!("no workflow files found in {}/", platform_dir).into())
    }
}

impl DetectWorkflowFilePort for DetectWorkflowFileService {
    fn execute(&self, request: DetectWorkflowFileRequest) -> Result<PathBuf, Box<dyn Error>> {
        let repo_path = request.repo_path();
        WORKFLOW_DIRECTORIES
            .iter()
            .find(|platform_dir| repo_path.join(platform_dir).exists())
            .map_or_else(
                || {
                    Err(format!(
                        "no workflows directory found ({})",
                        supported_workflows_display()
                    )
                    .into())
                },
                |platform_dir| self.first_workflow_file(repo_path.join(platform_dir), platform_dir),
            )
    }
}
