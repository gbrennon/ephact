use crate::application::dtos::{
    requests::BuildJobEnvironmentRequest, responses::BuildJobEnvironmentResponse,
};

/// Builds the environment variables for a job.
pub trait JobEnvironmentBuilderPort: Send + Sync {
    fn build(&self, request: BuildJobEnvironmentRequest) -> BuildJobEnvironmentResponse;
}
