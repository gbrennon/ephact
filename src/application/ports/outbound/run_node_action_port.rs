use crate::application::dtos::requests::RunNodeActionRequest;
use crate::application::dtos::responses::RunNodeActionResponse;
use crate::domain::errors::StepError;

/// Inbound port for running a JavaScript action inside the job's container.
pub trait RunNodeActionPort: Send + Sync {
    /// Copies the action in and runs its entry point with node.
    fn execute(
        &self,
        request: RunNodeActionRequest<'_>,
    ) -> Result<RunNodeActionResponse, StepError>;
}
