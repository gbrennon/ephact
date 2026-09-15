#[derive(Debug, thiserror::Error)]
pub enum LoadWorkflowError {
    #[error("{0}")]
    Parse(#[source] serde_yaml::Error),
    #[error("{0}")]
    Message(String),
}
