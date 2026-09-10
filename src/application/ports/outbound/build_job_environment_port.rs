use crate::application::dtos::requests::BuildJobEnvironmentRequest;
use crate::application::dtos::responses::BuildJobEnvironmentResponse;

/// Outbound port for building the environment variables a job runs with.
pub trait BuildJobEnvironmentPort: Send + Sync {
    /// Returns the merged environment for the job's execution.
    fn execute(&self, request: BuildJobEnvironmentRequest<'_>) -> BuildJobEnvironmentResponse;
}
