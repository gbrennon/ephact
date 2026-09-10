#![allow(dead_code)]
use std::error::Error;

use ephact::application::dtos::requests::RunAllWorkflowsRequest;
use ephact::application::dtos::responses::RunSummaryResponse;
use ephact::application::ports::inbound::run_all_workflows_port::RunAllWorkflowsPort;

pub struct StubRunAllWorkflowsPort {
    pub result: Result<RunSummaryResponse, String>,
}

impl RunAllWorkflowsPort for StubRunAllWorkflowsPort {
    fn execute(
        &self,
        _request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, Box<dyn Error>> {
        self.result.clone().map_err(Box::<dyn Error>::from)
    }
}
