use std::sync::Mutex;

use ephact::application::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::inbound::RunActionPort,
};

pub struct RecordingRunActionPort {
    pub requests: Mutex<Vec<RunActionRequest>>,
}

impl RecordingRunActionPort {
    pub fn new() -> Self {
        Self {
            requests: Mutex::new(Vec::new()),
        }
    }

    pub fn requests(&self) -> Vec<RunActionRequest> {
        self.requests.lock().expect("request lock").clone()
    }
}

impl RunActionPort for RecordingRunActionPort {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        self.requests.lock().expect("request lock").push(request);
        Ok(ExecuteActionResponse::note("action completed".to_owned()))
    }
}
