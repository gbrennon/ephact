#[derive(Debug)]
pub enum ListWorkflowsError {
    WorkflowSource(String),
}

impl_application_error!(
    ListWorkflowsError,
    |error: &ListWorkflowsError| match error {
        ListWorkflowsError::WorkflowSource(message) =>
            format!("listing workflows failed: {message}"),
    },
);

impl std::error::Error for ListWorkflowsError {}
