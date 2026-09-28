use crate::errors::WorkflowSourceError;

#[derive(Debug)]
pub enum DiscoverRunInputsError {
    WorkflowSource(WorkflowSourceError),
    Discovery(String),
}

impl_application_error!(
    DiscoverRunInputsError,
    |error: &DiscoverRunInputsError| match error {
        DiscoverRunInputsError::WorkflowSource(error) => format!("workflow source failed: {error}"),
        DiscoverRunInputsError::Discovery(message) =>
            format!("workflow input discovery failed: {message}"),
    },
);

impl std::error::Error for DiscoverRunInputsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WorkflowSource(error) => Some(error),
            Self::Discovery(_) => None,
        }
    }
}
