#![allow(dead_code)]
use ephact::{
    application::{
        dtos::requests::CopyRepositoryToContainerRequest, errors::CopyRepositoryToContainerError,
        ports::outbound::container_port::ContainerPort,
    },
    infrastructure::containers::CopyRepositoryToContainerPort,
};
use parking_lot::Mutex;

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
        request: CopyRepositoryToContainerRequest,
        _container: &dyn ContainerPort,
    ) -> Result<(), CopyRepositoryToContainerError> {
        let request_str = format!(
            "copy {} to {}",
            request.repo_path().display(),
            request.container_path()
        );
        self.requests.lock().push(request_str);

        if let Some(failure) = &self.failure {
            return Err(CopyRepositoryToContainerError::Container(failure.clone()));
        }

        Ok(())
    }
}
