/// Application-level failure reported while running one or more workflows.
///
#[derive(Debug)]
pub enum ApplicationError {
    /// A workflow operation failed. The contained string describes the failure.
    Workflow(String),
}

impl_application_error!(ApplicationError, |error: &ApplicationError| match error {
    ApplicationError::Workflow(message) => message.clone(),
},);

impl std::error::Error for ApplicationError {}
