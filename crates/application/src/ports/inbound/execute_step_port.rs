use crate::{
    dtos::{requests::ExecuteStepRequest, responses::ExecutedStepResponse},
    errors::ExecuteStepError,
};

/// Inbound port for executing one step of a job.
pub trait ExecuteStepPort: Send + Sync {
    /// Resolves the step's expressions and runs it as a script or an action.
    /// The response contains the resolved step and captured action output.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteStepError`] when the step cannot be decoded,
    /// interpolated, or executed.
    fn execute(
        &self,
        request: ExecuteStepRequest,
    ) -> Result<ExecutedStepResponse, ExecuteStepError>;
}
