#[derive(Debug)]
pub enum ExecuteWorkflowError {
    Workflow(String),
}

impl_application_error!(
    ExecuteWorkflowError,
    |error: &ExecuteWorkflowError| match error {
        ExecuteWorkflowError::Workflow(message) => message.clone(),
    },
);

impl std::error::Error for ExecuteWorkflowError {}
