use crate::application::{
    dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
    errors::ListWorkflowsError,
};

/// Inbound port for listing workflows in a repository.
pub trait ListWorkflowsPort {
    /// Discovers and lists all workflows found in the repository.
    fn execute(
        &self,
        request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, ListWorkflowsError>;
}
