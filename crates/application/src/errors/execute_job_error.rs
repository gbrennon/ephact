use crate::domain::errors::StepError;

#[derive(Debug, thiserror::Error)]
pub enum ExecuteJobError {
    #[error("{0}")]
    Step(#[source] StepError),
    #[error("{0}")]
    Preparation(String),
}
