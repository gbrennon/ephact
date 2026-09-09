use super::{
    list_all_workflow_files_port::ListAllWorkflowFilesPort,
    list_workflow_directory_port::ListWorkflowDirectoryPort,
};
use std::error::Error;

use super::workflow_directories::WORKFLOW_DIRECTORIES;
use crate::application::dtos::{
    ListAllWorkflowFilesRequest, ListAllWorkflowFilesResponse, ListWorkflowDirectoryRequest,
};

/// Service that lists every workflow file of a repository, `.forgejo` first.
pub struct ListAllWorkflowFilesService {
    directory_lister: Box<dyn ListWorkflowDirectoryPort>,
}

impl ListAllWorkflowFilesService {
    pub fn new(directory_lister: Box<dyn ListWorkflowDirectoryPort>) -> Self {
        Self { directory_lister }
    }
}

impl ListAllWorkflowFilesPort for ListAllWorkflowFilesService {
    fn execute(
        &self,
        request: ListAllWorkflowFilesRequest<'_>,
    ) -> Result<ListAllWorkflowFilesResponse, Box<dyn Error>> {
        let mut workflows = Vec::new();
        for platform_dir in &WORKFLOW_DIRECTORIES {
            let workflows_dir = request.repo_path().join(platform_dir);
            if workflows_dir.exists() {
                workflows.extend(
                    self.directory_lister
                        .execute(ListWorkflowDirectoryRequest::new(&workflows_dir))?
                        .workflow_files()
                        .iter()
                        .cloned(),
                );
            }
        }
        if workflows.is_empty() {
            return Err(
                "no workflow files found in .forgejo/workflows/ or .github/workflows/".into(),
            );
        }
        Ok(ListAllWorkflowFilesResponse::new(workflows))
    }
}
