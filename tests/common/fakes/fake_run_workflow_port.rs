#![allow(dead_code)]
use std::time::Duration;

use ephact::application::dtos::requests::RunWorkflowRequest;
use ephact::application::dtos::responses::RunSummaryResponse;
use ephact::application::ports::inbound::run_workflow_port::RunWorkflowPort;

pub struct FakeRunWorkflowPort {
    pub result: RunSummaryResponse,
}

impl FakeRunWorkflowPort {
    pub fn new(success: bool) -> Self {
        Self {
            result: RunSummaryResponse::new("test".to_string(), vec![], success, Duration::ZERO),
        }
    }
}

impl RunWorkflowPort for FakeRunWorkflowPort {
    fn execute(
        &self,
        _request: RunWorkflowRequest,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        Ok(self.result.clone())
    }
}
