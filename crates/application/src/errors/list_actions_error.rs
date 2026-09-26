#[derive(Debug, thiserror::Error)]
pub enum ListActionsError {
    #[error("listing actions failed: {0}")]
    WorkflowSource(String),
}
