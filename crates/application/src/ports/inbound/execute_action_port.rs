use crate::{
    dtos::{requests::ExecuteActionRequest, responses::ExecuteActionResponse},
    errors::ExecuteActionError,
};

/// Executes the action referenced by a workflow step.
///
/// Local references are resolved in the repository; remote references are
/// fetched before the action definition is loaded. Composite and JavaScript
/// actions are executed in the supplied container. Unsupported container
/// actions return an error rather than being skipped.
///
/// The response contains the action exit status and captured output. Step
/// failures retain output captured before the failure.
pub trait ExecuteActionPort: Send + Sync {
    /// Resolves and executes the requested action.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteActionError`] when the action cannot be resolved,
    /// loaded, prepared, or executed.
    fn execute(
        &self,
        request: ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, ExecuteActionError>;
}
