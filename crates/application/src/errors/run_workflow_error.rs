#[derive(Debug)]
pub enum RunWorkflowError {
    Workflow(String),
}

impl_application_error!(RunWorkflowError, |error: &RunWorkflowError| match error {
    RunWorkflowError::Workflow(message) => message.clone(),
},);

impl std::error::Error for RunWorkflowError {}
