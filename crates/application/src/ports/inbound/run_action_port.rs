use crate::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
};

/// Executes one action from a serialized workflow step.
pub trait RunActionPort {
    /// Decodes the requested step, resolves its action reference, and returns
    /// the action's exit code and captured output.
    ///
    /// # Errors
    ///
    /// Returns [`RunActionError`] when the step cannot be decoded or the action
    /// cannot be executed.
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError>;
}
