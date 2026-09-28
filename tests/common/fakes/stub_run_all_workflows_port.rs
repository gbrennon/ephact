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
    ) -> Result<RunSummaryResponse, ephact::application::errors::ApplicationError> {
        self.result
            .clone()
            .map_err(ephact::application::errors::ApplicationError::Workflow)
    }
}
