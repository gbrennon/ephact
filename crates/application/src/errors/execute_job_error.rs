use crate::domain::errors::StepError;

/// Failure while preparing or executing a job.
///
#[derive(Debug)]
pub enum ExecuteJobError {
    /// A step-level failure occurred during job execution.
    Step(StepError),
    /// Job preparation failed. The contained string describes the preparation failure.
    Preparation(String),
}

impl_application_error!(ExecuteJobError, |error: &ExecuteJobError| match error {
    ExecuteJobError::Step(error) => error.to_string(),
    ExecuteJobError::Preparation(message) => message.clone(),
},);

impl std::error::Error for ExecuteJobError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Step(error) => Some(error),
            Self::Preparation(_) => None,
        }
    }
}
