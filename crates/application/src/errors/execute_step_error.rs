use crate::domain::errors::StepError;

/// Failure while decoding, interpolating, or executing a workflow step.
///
#[derive(Debug)]
pub enum ExecuteStepError {
    /// The underlying step operation failed.
    Step(StepError),
}

impl_application_error!(ExecuteStepError, |error: &ExecuteStepError| match error {
    ExecuteStepError::Step(error) => error.to_string(),
},);

impl ExecuteStepError {
    /// Returns the message stored in the underlying [`StepError`].
    pub fn message(&self) -> &str {
        match self {
            Self::Step(error) => error.message(),
        }
    }

    /// Returns standard output captured by the underlying [`StepError`].
    pub fn stdout(&self) -> &str {
        match self {
            Self::Step(error) => error.stdout(),
        }
    }

    /// Returns standard error captured by the underlying [`StepError`].
    pub fn stderr(&self) -> &str {
        match self {
            Self::Step(error) => error.stderr(),
        }
    }
}

impl std::error::Error for ExecuteStepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Step(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_step_error_details() {
        let step_error = StepError::new("script failed").with_stdout("output");

        let error = ExecuteStepError::Step(step_error);

        assert_eq!(error.to_string(), "script failed");
        assert_eq!(
            std::error::Error::source(&error).unwrap().to_string(),
            "script failed"
        );
    }
}
