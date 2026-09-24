use ephact::application::{
    dtos::{
        requests::ListWorkflowsRequest,
        responses::{ListWorkflowsResponse, WorkflowListItemResponse},
    },
    errors::ListWorkflowsError,
    ports::inbound::list_workflows_port::ListWorkflowsPort,
};

pub struct FakeListWorkflowsPort {
    workflows: Vec<WorkflowListItemResponse>,
    error_message: Option<String>,
}

impl FakeListWorkflowsPort {
    pub fn new() -> Self {
        Self {
            workflows: vec![],
            error_message: None,
        }
    }

    pub fn with_workflows(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            workflows,
            error_message: None,
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            workflows: vec![],
            error_message: Some(message.to_string()),
        }
    }
}

impl Default for FakeListWorkflowsPort {
    fn default() -> Self {
        Self::new()
    }
}

impl ListWorkflowsPort for FakeListWorkflowsPort {
    fn execute(
        &self,
        _request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, ListWorkflowsError> {
        if let Some(error_message) = &self.error_message {
            return Err(ListWorkflowsError::WorkflowSource(error_message.clone()));
        }

        Ok(ListWorkflowsResponse::new(self.workflows.clone()))
    }
}
