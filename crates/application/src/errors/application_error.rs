#[derive(Debug)]
pub enum ApplicationError {
    Workflow(String),
}

impl_application_error!(ApplicationError, |error: &ApplicationError| match error {
    ApplicationError::Workflow(message) => message.clone(),
},);

impl std::error::Error for ApplicationError {}
