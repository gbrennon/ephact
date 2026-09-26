use ephact::application::{
    dtos::{requests::RunWorkflowRequest, responses::RunSummaryResponse},
    ports::inbound::run_workflow_port::RunWorkflowPort,
};

pub struct StubRunWorkflowPort {
    pub result: Result<RunSummaryResponse, String>,
}

impl RunWorkflowPort for StubRunWorkflowPort {
    fn execute(
        &self,
        _request: RunWorkflowRequest,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<
                        RunSummaryResponse,
                        ephact::application::errors::RunWorkflowError,
                    >,
                > + Send
                + '_,
        >,
    > {
        let result = self.result.clone();
        Box::pin(
            async move { result.map_err(ephact::application::errors::RunWorkflowError::Workflow) },
        )
    }
}
