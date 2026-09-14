#![allow(dead_code)]
use ephact::application::dtos::requests::CopyRepositoryToContainerRequest;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::infrastructure::containers::copy_repository_to_container_port::CopyRepositoryToContainerPort;
use parking_lot::Mutex;
use std::error::Error;

#[derive(Clone, Default)]
pub struct FakeCopyRepositoryToContainerPort {
    requests: std::sync::Arc<Mutex<Vec<String>>>,
    failure: Option<String>,
}

impl FakeCopyRepositoryToContainerPort {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn failing(message: &str) -> Self {
        Self {
            failure: Some(message.to_string()),
            ..Default::default()
        }
    }

    pub fn requests(&self) -> Vec<String> {
        self.requests.lock().clone()
    }
}

impl CopyRepositoryToContainerPort for FakeCopyRepositoryToContainerPort {
    fn execute(
        &self,
        request: CopyRepositoryToContainerRequest<'_>,
        _container: &dyn ContainerPort,
    ) -> Result<(), Box<dyn Error>> {
        let request_str = format!(
            "copy {} to {}",
            request.repo_path().display(),
            request.container_path()
        );
        self.requests.lock().push(request_str);

        if let Some(failure) = &self.failure {
            return Err(failure.clone().into());
        }

        Ok(())
    }
}
