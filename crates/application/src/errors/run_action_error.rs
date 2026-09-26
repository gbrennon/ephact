use crate::domain::errors::StepError;

#[derive(Debug, thiserror::Error)]
pub enum RunActionError {
    #[error("action execution failed: {0}")]
    Step(#[source] StepError),
}
