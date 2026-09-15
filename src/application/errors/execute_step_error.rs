use crate::domain::errors::StepError;

#[derive(Debug, thiserror::Error)]
pub enum ExecuteStepError {
    #[error("{0}")]
    Step(#[source] StepError),
}

impl ExecuteStepError {
    pub fn message(&self) -> &str {
        match self {
            Self::Step(error) => error.message(),
        }
    }

    pub fn stdout(&self) -> &str {
        match self {
            Self::Step(error) => error.stdout(),
        }
    }

    pub fn stderr(&self) -> &str {
        match self {
            Self::Step(error) => error.stderr(),
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
