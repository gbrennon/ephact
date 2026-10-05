use crate::{
    dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
    errors::ListWorkflowsError,
};

/// Inbound port for listing workflows in a repository.
pub trait ListWorkflowsPort {
    /// Discovers and lists all workflows found in the repository.
    /// Each response entry identifies a workflow file and its declared
    /// trigger events.
    ///
    /// # Errors
    ///
    /// Returns [`ListWorkflowsError`] when the repository cannot be resolved or
    /// its workflow source cannot be read.
    fn execute(
        &self,
        request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, ListWorkflowsError>;
}
