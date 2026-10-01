use std::path::PathBuf;

use crate::{
    application::{
        dtos::{requests::ListActionsRequest, responses::ListActionsResponse},
        ports::inbound::list_actions_port::ListActionsPort,
    },
    infrastructure::RepositoryResolver,
};

pub struct ListActionsHandler;

impl ListActionsHandler {
    /// Lists the actions referenced by the workflows under `repository_path`.
    pub fn handle(
        port: &dyn ListActionsPort,
        repository_path: PathBuf,
    ) -> Result<ListActionsResponse, Box<dyn std::error::Error>> {
        let repository = RepositoryResolver::resolve_from_path(repository_path)?;
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
