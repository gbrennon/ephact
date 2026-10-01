use crate::{
    domain::{RepoPath, Repository, RepositoryName},
    dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
    errors::ListWorkflowsError,
    ports::{inbound::list_workflows_port::ListWorkflowsPort, outbound::WorkflowSourcePort},
};

/// Application service implementing the `ListWorkflowsPort` entrypoint.
pub struct ListWorkflowsService {
    workflow_source: Box<dyn WorkflowSourcePort>,
}

impl ListWorkflowsService {
    pub fn new(workflow_source: Box<dyn WorkflowSourcePort>) -> Self {
        Self { workflow_source }
    }
}

impl ListWorkflowsPort for ListWorkflowsService {
    fn execute(
        &self,
        request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, ListWorkflowsError> {
        let path = RepoPath::new(request.repository_path().to_path_buf())
            .map_err(|error| ListWorkflowsError::WorkflowSource(error.to_string()))?;
        let name = RepositoryName::new(request.repository_name().to_owned())
            .map_err(|error| ListWorkflowsError::WorkflowSource(error.to_string()))?;
        let repository = Repository::new(path, name);
        let workflows = self
            .workflow_source
            .list_workflows(&repository)
            .map_err(|error| ListWorkflowsError::WorkflowSource(error.to_string()))?;
        Ok(ListWorkflowsResponse::new(workflows))
    }
}
