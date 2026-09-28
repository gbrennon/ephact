use crate::domain::errors::StepError;

#[derive(Debug)]
pub enum ExecuteNestedActionError {
    Step(StepError),
}

impl_application_error!(
    ExecuteNestedActionError,
    |error: &ExecuteNestedActionError| match error {
        ExecuteNestedActionError::Step(error) => format!("nested action execution failed: {error}"),
    },
);

impl std::error::Error for ExecuteNestedActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Step(error) => Some(error),
        }
    }
}
