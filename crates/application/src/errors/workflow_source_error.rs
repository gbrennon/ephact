use std::io;

#[derive(Debug)]
pub enum WorkflowSourceError {
    Io(io::Error),
    NotFound(String),
    Empty,
}

impl_application_error!(
    WorkflowSourceError,
    |error: &WorkflowSourceError| match error {
        WorkflowSourceError::Io(error) => error.to_string(),
        WorkflowSourceError::NotFound(message) => message.clone(),
        WorkflowSourceError::Empty => "no workflow files found".to_owned(),
    },
);

impl std::error::Error for WorkflowSourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::NotFound(_) | Self::Empty => None,
        }
    }
}
