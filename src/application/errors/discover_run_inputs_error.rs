use crate::application::errors::WorkflowSourceError;

#[derive(Debug, thiserror::Error)]
pub enum DiscoverRunInputsError {
    #[error("workflow source failed: {0}")]
    WorkflowSource(#[source] WorkflowSourceError),
    #[error("workflow input discovery failed: {0}")]
    Discovery(String),
}
