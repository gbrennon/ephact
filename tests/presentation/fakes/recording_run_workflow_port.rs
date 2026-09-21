use std::sync::Mutex;

use ephact::application::{
    dtos::{requests::RunWorkflowRequest, responses::RunSummaryResponse},
    errors::RunWorkflowError,
    ports::inbound::run_workflow_port::RunWorkflowPort,
};

pub struct RecordingRunWorkflowPort {
    summary: RunSummaryResponse,
    recorded_request: Mutex<Option<RunWorkflowRequest>>,
}

impl RecordingRunWorkflowPort {
    pub fn new(summary: RunSummaryResponse) -> Self {
        Self {
            summary,
            recorded_request: Mutex::new(None),
        }
    }

    pub fn recorded_request(&self) -> Option<RunWorkflowRequest> {
        self.recorded_request
            .lock()
            .expect("recording lock")
            .clone()
    }
}

impl RunWorkflowPort for RecordingRunWorkflowPort {
    fn execute(
        &self,
        request: RunWorkflowRequest,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<RunSummaryResponse, RunWorkflowError>>
                + Send
                + '_,
        >,
    > {
        *self.recorded_request.lock().expect("recording lock") = Some(request);
        let summary = self.summary.clone();
        Box::pin(async move { Ok(summary) })
    }
}
