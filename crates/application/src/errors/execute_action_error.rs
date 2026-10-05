use crate::domain::errors::{ActionError, StepError};

/// Failure while resolving or executing an action request.
///
#[derive(Debug)]
pub enum ExecuteActionError {
    /// A step-level failure occurred while resolving or running the action.
    Step(StepError),
    /// An action-level failure was reported while handling the request.
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
    /// Returns the message rendered by this error's display implementation.
    pub fn message(&self) -> String {
        self.to_string()
    }

    /// Returns standard output captured by the nested [`StepError`], or an empty
    /// string for [`ExecuteActionError::Action`].
    pub fn stdout(&self) -> &str {
        match self {
            Self::Step(error) => error.stdout(),
            Self::Action(_) => "",
        }
    }

    /// Returns standard error captured by the nested [`StepError`], or an empty
    /// string for [`ExecuteActionError::Action`].
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
