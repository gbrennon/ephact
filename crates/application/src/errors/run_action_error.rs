use crate::domain::errors::StepError;

#[derive(Debug)]
pub enum RunActionError {
    Step(StepError),
}

impl_application_error!(RunActionError, |error: &RunActionError| match error {
    RunActionError::Step(error) => format!("action execution failed: {error}"),
},);

impl std::error::Error for RunActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Step(error) => Some(error),
        }
    }
}
