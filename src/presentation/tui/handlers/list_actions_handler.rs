use std::path::PathBuf;

use crate::application::dtos::requests::ListActionsRequest;
use crate::application::dtos::responses::ListActionsResponse;
use crate::application::ports::inbound::list_actions_port::ListActionsPort;
use crate::domain::{RepoPath, Repository, RepositoryName};

pub struct ListActionsHandler {}

impl ListActionsHandler {
    pub fn handle(
        port: &dyn ListActionsPort,
        repository_path: PathBuf,
    ) -> Result<ListActionsResponse, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(repository_path).map_err(|e| format!("{e:?}"))?;
        let repo_name = RepositoryName::from_repo_path(&repo_path).map_err(|e| format!("{e:?}"))?;
        let repository = Repository::new(repo_path, repo_name);
        let request = ListActionsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        );
        let response = port.execute(request)?;
        Ok(response)
    }
}
