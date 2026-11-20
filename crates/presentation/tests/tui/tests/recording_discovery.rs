use std::sync::Mutex;

use ephact::application::{
    dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
    errors::DiscoverRunInputsError,
    ports::outbound::RunInputsDiscovererPort,
};

pub struct RecordingDiscovery {
    requests: Mutex<Vec<String>>,
}

impl RecordingDiscovery {
    pub fn new() -> Self {
        Self {
            requests: Mutex::new(Vec::new()),
        }
    }

    pub fn requests(&self) -> Vec<String> {
        self.requests.lock().expect("discovery lock").clone()
    }
}

impl RunInputsDiscovererPort for RecordingDiscovery {
    fn discover(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        self.requests
            .lock()
            .expect("discovery lock")
            .push(request.action_ref().unwrap_or_default().to_owned());
        Ok(Vec::new())
    }
}
