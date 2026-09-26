#[derive(Debug, thiserror::Error)]
pub enum RunAllWorkflowsError {
    #[error("{0}")]
    Workflow(String),
}
