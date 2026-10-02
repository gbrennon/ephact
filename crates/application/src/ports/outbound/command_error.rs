use crate::{
    domain::errors::StepError,
    errors::{ExecuteJobError, ExecuteWorkflowError},
};

#[derive(Debug)]
pub enum CommandError {
    Workflow(ExecuteWorkflowError),
    Job(ExecuteJobError),
    Step(StepError),
    Transport(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Workflow(error) => write!(formatter, "{error}"),
            Self::Job(error) => write!(formatter, "{error}"),
            Self::Step(error) => write!(formatter, "{error}"),
            Self::Transport(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for CommandError {}
