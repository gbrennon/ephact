use crate::errors::WorkflowSourceError;

/// Failure while discovering inputs declared by workflows and local actions.
///
#[derive(Debug)]
pub enum DiscoverRunInputsError {
    /// The workflow source could not provide the workflow content needed for discovery.
    WorkflowSource(WorkflowSourceError),
    /// Workflow or action input discovery failed after source content was obtained.
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
