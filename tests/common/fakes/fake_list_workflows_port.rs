#![allow(dead_code)]
use ephact::application::dtos::requests::ListWorkflowsRequest;
use ephact::application::dtos::responses::ListWorkflowsResponse;
use ephact::application::ports::inbound::list_workflows_port::ListWorkflowsPort;

pub struct FakeListWorkflowsPort;

impl FakeListWorkflowsPort {
    pub fn new() -> Self {
        Self
    }
}

impl ListWorkflowsPort for FakeListWorkflowsPort {
    fn execute(
        &self,
        _request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, Box<dyn std::error::Error>> {
        Ok(ListWorkflowsResponse::new(vec![]))
    }
}
