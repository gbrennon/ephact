use std::error::Error;

use super::{
    list_all_workflow_files_port::ListAllWorkflowFilesPort,
    list_workflow_directory_port::ListWorkflowDirectoryPort,
    workflow_directories::{WORKFLOW_DIRECTORIES, supported_workflows_display},
};
use crate::application::dtos::{
    requests::{ListAllWorkflowFilesRequest, ListWorkflowDirectoryRequest},
    responses::ListAllWorkflowFilesResponse,
};

/// Service that lists every workflow file of a repository, `.forgejo` first.
pub struct ListAllWorkflowFilesService {
    directory_lister: Box<dyn ListWorkflowDirectoryPort>,
}

impl ListAllWorkflowFilesService {
    pub fn new(directory_lister: Box<dyn ListWorkflowDirectoryPort>) -> Self {
        Self { directory_lister }
    }

    fn response_from_workflows(
        workflows: Vec<std::path::PathBuf>,
    ) -> Result<ListAllWorkflowFilesResponse, Box<dyn Error>> {
        match workflows.is_empty() {
            true => Err(format!(
                "no workflow files found in {}",
                supported_workflows_display()
            )
            .into()),
            false => Ok(ListAllWorkflowFilesResponse::new(workflows)),
        }
    }
}

impl ListAllWorkflowFilesPort for ListAllWorkflowFilesService {
    fn execute(
        &self,
        request: ListAllWorkflowFilesRequest,
    ) -> Result<ListAllWorkflowFilesResponse, Box<dyn Error>> {
        let repo_path = request.repo_path();
        let workflows = WORKFLOW_DIRECTORIES
            .iter()
            .filter_map(|platform_dir| {
                let workflows_dir = repo_path.join(platform_dir);
                workflows_dir.exists().then_some(workflows_dir)
            })
            .map(|workflows_dir| {
                self.directory_lister
                    .execute(ListWorkflowDirectoryRequest::new(workflows_dir))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|responses| {
                responses
                    .into_iter()
                    .flat_map(|response| response.workflow_files().to_vec())
                    .collect::<Vec<_>>()
            })?;

        Self::response_from_workflows(workflows)
    }
}
