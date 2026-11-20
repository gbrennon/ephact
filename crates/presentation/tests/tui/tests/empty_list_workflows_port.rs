use crate::application::{
    dtos::{requests::ListWorkflowsRequest, responses::ListWorkflowsResponse},
    errors::ListWorkflowsError,
    ports::inbound::ListWorkflowsPort,
};

pub struct EmptyListWorkflowsPort;

impl ListWorkflowsPort for EmptyListWorkflowsPort {
    fn execute(
        &self,
        _request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, ListWorkflowsError> {
        Ok(ListWorkflowsResponse::new(Vec::new()))
    }
}
