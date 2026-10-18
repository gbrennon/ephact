/// Failure while loading, planning, or executing a workflow.
///
#[derive(Debug)]
pub enum ExecuteWorkflowError {
    /// A workflow operation failed. The contained string describes the failure.
    Workflow(String),
    /// The requested job does not exist in the workflow.
    UnknownJob { job: String },
}

impl_application_error!(
    ExecuteWorkflowError,
    |error: &ExecuteWorkflowError| match error {
        ExecuteWorkflowError::Workflow(message) => message.clone(),
        ExecuteWorkflowError::UnknownJob { job } => {
            format!("selected job '{job}' was not found")
        }
    },
);

impl std::error::Error for ExecuteWorkflowError {}
