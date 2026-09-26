use crate::domain::errors::{ActionError, StepError};

#[derive(Debug, thiserror::Error)]
pub enum ExecuteActionError {
    #[error("{0}")]
    Step(#[source] StepError),
    #[error("{0}")]
    Action(#[source] ActionError),
}

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
