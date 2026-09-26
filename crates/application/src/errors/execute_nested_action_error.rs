use crate::domain::errors::StepError;

#[derive(Debug, thiserror::Error)]
pub enum ExecuteNestedActionError {
    #[error("nested action execution failed: {0}")]
    Step(#[source] StepError),
}
