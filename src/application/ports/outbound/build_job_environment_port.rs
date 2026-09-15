use crate::application::dtos::requests::BuildJobEnvironmentRequest;
use crate::application::dtos::responses::BuildJobEnvironmentResponse;

/// Outbound port for building the environment variables a job runs with.
pub trait BuildJobEnvironmentPort: Send + Sync {
    fn execute(&self, request: BuildJobEnvironmentRequest) -> BuildJobEnvironmentResponse;
}
