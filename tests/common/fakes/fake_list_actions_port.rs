use ephact::application::{
    dtos::{requests::ListActionsRequest, responses::ListActionsResponse},
    errors::ListActionsError,
    ports::inbound::list_actions_port::ListActionsPort,
};

#[derive(Clone)]
pub struct FakeListActionsPort {
    actions: Vec<String>,
    error_message: Option<String>,
}

impl FakeListActionsPort {
    pub fn new() -> Self {
        Self {
            actions: vec![],
            error_message: None,
        }
    }

    pub fn with_actions(actions: Vec<String>) -> Self {
        Self {
            actions,
            error_message: None,
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            actions: vec![],
            error_message: Some(message.to_string()),
        }
    }
}

impl Default for FakeListActionsPort {
    fn default() -> Self {
        Self::new()
    }
}

impl ListActionsPort for FakeListActionsPort {
    fn execute(
        &self,
        _request: ListActionsRequest,
    ) -> Result<ListActionsResponse, ListActionsError> {
        if let Some(error_message) = &self.error_message {
            return Err(ListActionsError::WorkflowSource(error_message.clone()));
        }

        Ok(ListActionsResponse::new(self.actions.clone()))
    }
}
