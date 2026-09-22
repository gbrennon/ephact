use std::path::PathBuf;

use crate::{
    application::{
        dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
        ports::inbound::list_workflows_port::ListWorkflowsPort,
    },
    domain::{RepoPath, Repository, RepositoryName},
};

pub struct ListWorkflowsHandler;

impl ListWorkflowsHandler {
    /// Lists the workflows discovered under `repository_path`.
    ///
    /// Converts the path into a [`Repository`], asks the port to list its
    /// workflows, and returns the raw response for the caller to render.
    pub fn handle(
        port: &dyn ListWorkflowsPort,
        repository_path: PathBuf,
    ) -> Result<ListWorkflowsResponse, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(repository_path)?;
        let repo_name = RepositoryName::from_repo_path(&repo_path)?;
        let repository = Repository::new(repo_path, repo_name);
        let request = ListWorkflowsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        );
        Ok(port.execute(request)?)
    }

    /// Formats the workflow names as a newline-separated list.
    pub fn render(response: &ListWorkflowsResponse) -> String {
        response
            .workflows()
            .iter()
            .map(|workflow| workflow.name().unwrap_or("Unnamed workflow").to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
