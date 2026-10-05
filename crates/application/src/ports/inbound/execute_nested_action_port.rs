use crate::{
    dtos::{requests::ExecuteActionRequest, responses::ExecuteActionResponse},
    errors::ExecuteNestedActionError,
};

/// Inbound port for recursively executing nested actions inside a composite action.
pub trait ExecuteNestedActionPort: Send + Sync {
    /// Runs a nested action and returns its response.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteNestedActionError`] when nested action execution fails.
    fn execute(
        &self,
        request: ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, ExecuteNestedActionError>;
}
