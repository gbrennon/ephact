#[derive(Debug)]
pub enum RunAllWorkflowsError {
    Workflow(String),
}

impl_application_error!(
    RunAllWorkflowsError,
    |error: &RunAllWorkflowsError| match error {
        RunAllWorkflowsError::Workflow(message) => message.clone(),
    },
);

impl std::error::Error for RunAllWorkflowsError {}
