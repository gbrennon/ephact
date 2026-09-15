#[derive(Debug, thiserror::Error)]
pub enum ExecuteWorkflowError {
    #[error("{0}")]
    Workflow(String),
}
