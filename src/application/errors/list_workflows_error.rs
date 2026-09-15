#[derive(Debug, thiserror::Error)]
pub enum ListWorkflowsError {
    #[error("listing workflows failed: {0}")]
    WorkflowSource(String),
}
