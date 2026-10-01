use std::path::PathBuf;

use ephact::application::{
    dtos::requests::ResolveNamedWorkflowFileRequest, ports::outbound::ResolveNamedWorkflowFilePort,
};
use parking_lot::Mutex;

/// Resolves every name to a prepared path, recording the names it was asked for.
pub struct FakeResolveNamedWorkflowFilePort {
    result: Result<PathBuf, String>,
    pub requested_names: Mutex<Vec<String>>,
}

impl FakeResolveNamedWorkflowFilePort {
    pub fn returning(path: PathBuf) -> Self {
        Self {
            result: Ok(path),
            requested_names: Mutex::new(Vec::new()),
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            result: Err(message.to_string()),
            requested_names: Mutex::new(Vec::new()),
        }
    }
}

impl ResolveNamedWorkflowFilePort for FakeResolveNamedWorkflowFilePort {
    fn resolve(
        &self,
        request: ResolveNamedWorkflowFileRequest,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        self.requested_names
            .lock()
            .push(request.workflow_name().to_string());
        self.result.clone().map_err(Into::into)
    }
}
