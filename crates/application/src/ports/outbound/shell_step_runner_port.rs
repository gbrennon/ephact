use crate::{
    domain::errors::StepError,
    dtos::{requests::RunShellStepRequest, responses::ExecResultResponse},
};

/// Runs a shell script and returns its process result.
pub trait ShellStepRunnerPort: Send + Sync {
    /// Runs the step's script and returns what the container reported.
    fn run(&self, request: RunShellStepRequest<'_>) -> Result<ExecResultResponse, StepError>;
}
