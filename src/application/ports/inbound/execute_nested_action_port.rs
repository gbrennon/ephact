use crate::application::dtos::requests::ExecuteActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::domain::errors::StepError;

/// Inbound port for recursively executing nested actions inside a composite action.
pub trait ExecuteNestedActionPort: Send + Sync {
    /// Runs a nested action and returns its response.
    fn execute(
        &self,
        request: ExecuteActionRequest<'_>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
