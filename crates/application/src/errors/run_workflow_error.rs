#[derive(Debug, thiserror::Error)]
pub enum RunWorkflowError {
    #[error("{0}")]
    Workflow(String),
}
