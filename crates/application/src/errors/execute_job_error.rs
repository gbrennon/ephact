use crate::domain::errors::StepError;

#[derive(Debug)]
pub enum ExecuteJobError {
    Step(StepError),
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
