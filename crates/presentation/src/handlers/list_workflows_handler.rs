use std::path::PathBuf;

use crate::{
    application::{
        dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
        ports::inbound::list_workflows_port::ListWorkflowsPort,
    },
    infrastructure::RepositoryResolver,
};

pub struct ListWorkflowsHandler;

impl ListWorkflowsHandler {
    pub fn handle(
        port: &dyn ListWorkflowsPort,
        repository_path: PathBuf,
    ) -> Result<ListWorkflowsResponse, Box<dyn std::error::Error>> {
        let repository = RepositoryResolver::resolve_from_path(repository_path)?;
        let request = ListWorkflowsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        );
        Ok(port.execute(request)?)
    }

    pub fn render(response: &ListWorkflowsResponse) -> String {
        response
            .workflows()
            .iter()
            .map(|workflow| workflow.name().unwrap_or("Unnamed workflow").to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
