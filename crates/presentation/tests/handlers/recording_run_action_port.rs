use std::sync::Mutex;

use ephact::application::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::inbound::RunActionPort,
};

pub struct RecordingRunActionPort {
    response: ExecuteActionResponse,
    request: Mutex<Option<RunActionRequest>>,
}

impl RecordingRunActionPort {
    pub fn new(response: ExecuteActionResponse) -> Self {
        Self {
            response,
            request: Mutex::new(None),
        }
    }

    pub fn request(&self) -> RunActionRequest {
        self.request
            .lock()
            .expect("recording lock")
            .take()
            .expect("recorded request")
    }
}

impl RunActionPort for RecordingRunActionPort {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        *self.request.lock().expect("recording lock") = Some(request);
        Ok(self.response.clone())
    }
}
