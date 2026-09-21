use crate::{
    application::{
        dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
        errors::ListWorkflowsError,
        ports::{inbound::list_workflows_port::ListWorkflowsPort, outbound::WorkflowSourcePort},
    },
    domain::services::repository_factory::RepositoryFactory,
};

/// Application service implementing the `ListWorkflowsPort` entrypoint.
///
/// Agnostic by construction: it states the intent ("list the workflows of this
/// repository") and delegates every storage and parsing concern to the outbound
/// [`WorkflowSourcePort`].
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
        let repository = RepositoryFactory::create(
            request.repository_path().to_path_buf(),
            request.repository_name().to_string(),
        )
        .map_err(|error| ListWorkflowsError::WorkflowSource(format!("{error:?}")))?;
        let workflows = self
            .workflow_source
            .list_workflows(&repository)
            .map_err(|error| ListWorkflowsError::WorkflowSource(error.to_string()))?;
        Ok(ListWorkflowsResponse::new(workflows))
    }
}
