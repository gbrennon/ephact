use crate::application::dtos::requests::ExecuteStepRequest;
use crate::application::dtos::responses::ExecutedStepResponse;
use crate::application::errors::ExecuteStepError;

/// Inbound port for executing one step of a job.
pub trait ExecuteStepPort: Send + Sync {
    /// Resolves the step's expressions and runs it as a script or an action.
    fn execute(
        &self,
        request: ExecuteStepRequest,
    ) -> Result<ExecutedStepResponse, ExecuteStepError>;
}
