#![allow(dead_code)]

use ephact::application::dtos::requests::RunWorkflowRequest;
use ephact::application::dtos::responses::RunSummaryResponse;
use ephact::application::ports::inbound::run_workflow_port::RunWorkflowPort;

pub struct StubRunWorkflowPort {
    pub result: Result<RunSummaryResponse, String>,
}

impl RunWorkflowPort for StubRunWorkflowPort {
    fn execute(
        &self,
        _request: RunWorkflowRequest,
    ) -> Result<RunSummaryResponse, ephact::application::errors::RunWorkflowError> {
        self.result
            .clone()
            .map_err(ephact::application::errors::RunWorkflowError::Workflow)
    }
}
