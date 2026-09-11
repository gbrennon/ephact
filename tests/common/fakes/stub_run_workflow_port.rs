#![allow(dead_code)]
use std::error::Error;

use ephact::application::dtos::requests::RunWorkflowRequest;
use ephact::application::dtos::responses::RunSummaryResponse;
use ephact::application::ports::inbound::run_workflow_port::RunWorkflowPort;

pub struct StubRunWorkflowPort {
    pub result: Result<RunSummaryResponse, String>,
}

impl RunWorkflowPort for StubRunWorkflowPort {
    fn execute(&self, _request: RunWorkflowRequest) -> Result<RunSummaryResponse, Box<dyn Error>> {
        self.result.clone().map_err(Box::<dyn Error>::from)
    }
}
