use std::cell::RefCell;

use ephact::{
    application::dtos::requests::RunWorkflowRequest,
    application::dtos::responses::RunSummaryResponse, application::errors::RunWorkflowError,
    application::ports::inbound::run_workflow_port::RunWorkflowPort,
};

pub struct RecordingRunWorkflowPort {
    summary: RunSummaryResponse,
    recorded_request: RefCell<Option<RunWorkflowRequest>>,
}

impl RecordingRunWorkflowPort {
    pub fn new(summary: RunSummaryResponse) -> Self {
        Self {
            summary,
            recorded_request: RefCell::new(None),
        }
    }

    pub fn recorded_request(&self) -> Option<RunWorkflowRequest> {
        self.recorded_request.borrow().clone()
    }
}

impl RunWorkflowPort for RecordingRunWorkflowPort {
    fn execute(&self, request: RunWorkflowRequest) -> Result<RunSummaryResponse, RunWorkflowError> {
        *self.recorded_request.borrow_mut() = Some(request);
        Ok(self.summary.clone())
    }
}
