use crate::application::dtos::requests::ListActionsRequest;
use crate::application::dtos::responses::ListActionsResponse;
use crate::application::errors::ListActionsError;
use crate::application::ports::inbound::list_actions_port::ListActionsPort;
use crate::application::ports::outbound::WorkflowSourcePort;
use crate::domain::services::repository_factory::RepositoryFactory;

/// Application service implementing the `ListActionsPort` entrypoint.
///
/// Agnostic by construction: it states the intent ("list the actions of this
/// repository") and delegates every storage and parsing concern to the outbound
/// [`WorkflowSourcePort`].
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
        let repository = RepositoryFactory::create(
            request.repository_path().to_path_buf(),
            request.repository_name().to_string(),
        )
        .map_err(|error| ListActionsError::WorkflowSource(format!("{error:?}")))?;
        let actions = self
            .workflow_source
            .list_actions(&repository)
            .map_err(|error| ListActionsError::WorkflowSource(error.to_string()))?;
        Ok(ListActionsResponse::new(actions))
    }
}
