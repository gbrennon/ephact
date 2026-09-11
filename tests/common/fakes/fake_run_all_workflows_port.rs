#![allow(dead_code)]
use std::time::Duration;

use ephact::application::dtos::requests::RunAllWorkflowsRequest;
use ephact::application::dtos::responses::RunSummaryResponse;
use ephact::application::ports::inbound::run_all_workflows_port::RunAllWorkflowsPort;

pub struct FakeRunAllWorkflowsPort {
    pub result: RunSummaryResponse,
}

impl FakeRunAllWorkflowsPort {
    pub fn new(success: bool) -> Self {
        Self {
            result: RunSummaryResponse::new(
                "All Workflows".to_string(),
                vec![],
                success,
                Duration::ZERO,
            ),
        }
    }
}

impl RunAllWorkflowsPort for FakeRunAllWorkflowsPort {
    fn execute(
        &self,
        _request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        Ok(self.result.clone())
    }
}
