use crate::{
    domain::{RepoPath, Repository, RepositoryName},
    dtos::{requests::ListActionsRequest, responses::ListActionsResponse},
    errors::ListActionsError,
    ports::{inbound::list_actions_port::ListActionsPort, outbound::WorkflowSourcePort},
};

/// Application service implementing the `ListActionsPort` entrypoint.
pub struct ListActionsService {
    workflow_source: Box<dyn WorkflowSourcePort>,
}

impl ListActionsService {
    pub fn new(workflow_source: Box<dyn WorkflowSourcePort>) -> Self {
        Self { workflow_source }
    }
}

impl ListActionsPort for ListActionsService {
    fn execute(
        &self,
        request: ListActionsRequest,
    ) -> Result<ListActionsResponse, ListActionsError> {
        let path = RepoPath::new(request.repository_path().to_path_buf())
            .map_err(|error| ListActionsError::WorkflowSource(error.to_string()))?;
        let name = RepositoryName::new(request.repository_name().to_owned())
            .map_err(|error| ListActionsError::WorkflowSource(error.to_string()))?;
        let repository = Repository::new(path, name);
        let actions = self
            .workflow_source
            .list_actions(&repository)
            .map_err(|error| ListActionsError::WorkflowSource(error.to_string()))?;
        Ok(ListActionsResponse::new(actions))
    }
}
