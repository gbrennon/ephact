use ephact::application::{
    dtos::{requests::RunAllWorkflowsRequest, responses::RunSummaryResponse},
    ports::inbound::run_all_workflows_port::RunAllWorkflowsPort,
};

pub struct StubRunAllWorkflowsPort {
    pub result: Result<RunSummaryResponse, String>,
}

impl RunAllWorkflowsPort for StubRunAllWorkflowsPort {
    fn execute(
        &self,
        _request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, ephact::application::errors::RunAllWorkflowsError> {
        self.result
            .clone()
            .map_err(ephact::application::errors::RunAllWorkflowsError::Workflow)
    }
}
