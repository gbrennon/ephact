#[derive(Debug)]
pub enum LoadWorkflowError {
    Parse(String),
    Message(String),
}

impl_application_error!(LoadWorkflowError, |error: &LoadWorkflowError| match error {
    LoadWorkflowError::Parse(message) | LoadWorkflowError::Message(message) => message.clone(),
},);

impl std::error::Error for LoadWorkflowError {}
