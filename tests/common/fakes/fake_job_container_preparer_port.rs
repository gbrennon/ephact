use std::sync::Arc;

use ephact::application::{
    dtos::{requests::PrepareJobContainerRequest, responses::PreparedJobContainerResponse},
    errors::PrepareJobContainerError,
    ports::outbound::job_container_preparer_port::JobContainerPreparerPort,
};
use parking_lot::Mutex;

use super::stub_container::StubContainer;

/// Prepares a stub container under a prepared name, or fails as configured.
#[derive(Clone)]
pub struct FakeJobContainerPreparerPort {
    container_name: String,
    failure: Option<String>,
    job_ids: Arc<Mutex<Vec<String>>>,
}

impl FakeJobContainerPreparerPort {
    pub fn named(container_name: &str) -> Self {
        Self {
            container_name: container_name.to_string(),
            failure: None,
            job_ids: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            container_name: String::new(),
            failure: Some(message.to_string()),
            job_ids: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn job_ids(&self) -> Vec<String> {
        self.job_ids.lock().clone()
    }
}

impl JobContainerPreparerPort for FakeJobContainerPreparerPort {
    fn prepare(
        &self,
        request: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError> {
        self.job_ids.lock().push(request.job_id().to_string());
        if let Some(message) = &self.failure {
            return Err(PrepareJobContainerError::Container(message.clone()));
        }
        Ok(PreparedJobContainerResponse::new(
            Box::new(StubContainer),
            self.container_name.clone(),
        ))
    }
}
