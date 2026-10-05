/// Failure while loading workflow content into the domain model.
///
#[derive(Debug)]
pub enum LoadWorkflowError {
    /// The workflow content could not be parsed.
    Parse(String),
    /// A workflow-loading operation failed with the contained message.
    Message(String),
}

impl_application_error!(LoadWorkflowError, |error: &LoadWorkflowError| match error {
    LoadWorkflowError::Parse(message) | LoadWorkflowError::Message(message) => message.clone(),
},);

impl std::error::Error for LoadWorkflowError {}
