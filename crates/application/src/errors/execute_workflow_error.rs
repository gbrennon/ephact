/// Failure while loading, planning, or executing a workflow.
///
#[derive(Debug)]
pub enum ExecuteWorkflowError {
    /// A workflow operation failed. The contained string describes the failure.
    Workflow(String),
}

impl_application_error!(
    ExecuteWorkflowError,
    |error: &ExecuteWorkflowError| match error {
        ExecuteWorkflowError::Workflow(message) => message.clone(),
    },
);

impl std::error::Error for ExecuteWorkflowError {}
