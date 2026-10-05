use std::io;

/// Failure while locating, reading, or listing workflow source files.
///
#[derive(Debug)]
pub enum WorkflowSourceError {
    /// A filesystem operation needed to access workflow source files failed.
    Io(io::Error),
    /// The requested workflow name did not match an available workflow file.
    NotFound(String),
    /// No workflow files were found for an unnamed workflow request.
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
