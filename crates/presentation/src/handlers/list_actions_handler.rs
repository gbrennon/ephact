use std::path::PathBuf;

use crate::{
    application::{
        dtos::{requests::ListActionsRequest, responses::ListActionsResponse},
        ports::inbound::list_actions_port::ListActionsPort,
    },
    domain::{RepoPath, Repository, RepositoryName},
};

pub struct ListActionsHandler;

impl ListActionsHandler {
    /// Lists the actions referenced by the workflows under `repository_path`.
    ///
    /// Converts the path into a [`Repository`], asks the port to list its
    /// actions, and returns the raw response for the caller to render.
    pub fn handle(
        port: &dyn ListActionsPort,
        repository_path: PathBuf,
    ) -> Result<ListActionsResponse, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(repository_path)?;
        let repo_name = RepositoryName::from_repo_path(&repo_path)?;
        let repository = Repository::new(repo_path, repo_name);
        let request = ListActionsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        );
        Ok(port.execute(request)?)
    }

    /// Formats the action references as a newline-separated list, keeping only
    /// the final path segment of each reference.
    pub fn render(response: &ListActionsResponse) -> String {
        response
            .actions()
            .iter()
            .map(|action| action.rsplit('/').next().unwrap_or(action).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
