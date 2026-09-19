use std::path::PathBuf;

use crate::application::dtos::requests::ListWorkflowsRequest;
use crate::application::dtos::responses::ListWorkflowsResponse;
use crate::application::ports::inbound::list_workflows_port::ListWorkflowsPort;
use crate::domain::{RepoPath, Repository, RepositoryName};

pub struct ListWorkflowsHandler;

impl ListWorkflowsHandler {
    pub fn handle(
        port: &dyn ListWorkflowsPort,
        repository_path: PathBuf,
    ) -> Result<ListWorkflowsResponse, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(repository_path).map_err(|e| format!("{e:?}"))?;
        let repo_name = RepositoryName::from_repo_path(&repo_path).map_err(|e| format!("{e:?}"))?;
        let repository = Repository::new(repo_path, repo_name);
        let request = ListWorkflowsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        );
        let response = port.execute(request)?;
        Ok(response)
    }
}
