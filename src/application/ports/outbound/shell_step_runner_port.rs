use crate::{
    application::dtos::{requests::RunShellStepRequest, responses::ExecResultResponse},
    domain::errors::StepError,
};

/// Runs a shell script and returns its process result.
pub trait ShellStepRunnerPort: Send + Sync {
    /// Runs the step's script and returns what the container reported.
    fn run(&self, request: RunShellStepRequest<'_>) -> Result<ExecResultResponse, StepError>;
}
