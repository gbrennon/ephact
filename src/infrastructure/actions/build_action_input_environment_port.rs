use crate::application::dtos::requests::BuildActionInputEnvironmentRequest;
use crate::application::dtos::responses::BuildActionInputEnvironmentResponse;

/// Outbound port for exposing an action's inputs as environment variables.
pub trait BuildActionInputEnvironmentPort: Send + Sync {
    /// Returns the environment with action inputs and action path configured.
    fn execute(
        &self,
        request: BuildActionInputEnvironmentRequest<'_>,
    ) -> BuildActionInputEnvironmentResponse;
}
