use crate::{
    domain::aggregates::Workflow, dtos::requests::LoadWorkflowRequest, errors::LoadWorkflowError,
};
/// Loads and parses a workflow document.
pub trait WorkflowLoaderPort: Send + Sync {
    /// Parses the workflow content and attaches the request's file name to the
    /// resulting [`Workflow`].
    ///
    /// # Errors
    ///
    /// Returns [`LoadWorkflowError`] when the workflow content is invalid.
    fn load(&self, request: LoadWorkflowRequest) -> Result<Workflow, LoadWorkflowError>;
}
