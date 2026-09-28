use crate::domain::errors::{ActionError, StepError};

#[derive(Debug)]
pub enum ExecuteActionError {
    Step(StepError),
    Action(ActionError),
}

impl_application_error!(
    ExecuteActionError,
    |error: &ExecuteActionError| match error {
        ExecuteActionError::Step(error) => error.to_string(),
        ExecuteActionError::Action(error) => error.to_string(),
    },
);

impl ExecuteActionError {
    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn stdout(&self) -> &str {
        match self {
            Self::Step(error) => error.stdout(),
            Self::Action(_) => "",
        }
    }

    pub fn stderr(&self) -> &str {
        match self {
            Self::Step(error) => error.stderr(),
            Self::Action(_) => "",
        }
    }
}

impl std::error::Error for ExecuteActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Step(error) => Some(error),
            Self::Action(error) => Some(error),
        }
    }
}
