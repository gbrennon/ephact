use std::{process, time::SystemTime};

use crate::application::{
    dtos::{
        requests::{
            CopyRepositoryToContainerRequest, CreateJobContainerRequest,
            PrepareJobContainerRequest, PullJobImageRequest,
        },
        responses::PreparedJobContainerResponse,
    },
    errors::PrepareJobContainerError,
    ports::outbound::{
        CopyRepositoryToContainerPort, CreateJobContainerPort, JobContainerPreparerPort,
        PullJobImagePort,
    },
};

const DEFAULT_CONTAINER_IMAGE: &str = "ubuntu:24.04";

/// Prepares isolated job containers from generic container image requests.
pub struct PrepareJobContainerService {
    image_puller: Box<dyn PullJobImagePort>,
    container_creator: Box<dyn CreateJobContainerPort>,
    repository_copier: Box<dyn CopyRepositoryToContainerPort>,
}

impl PrepareJobContainerService {
    /// Creates a service with image, container, and repository adapters.
    pub fn new(
        image_puller: Box<dyn PullJobImagePort>,
        container_creator: Box<dyn CreateJobContainerPort>,
        repository_copier: Box<dyn CopyRepositoryToContainerPort>,
    ) -> Self {
        Self {
            image_puller,
            container_creator,
            repository_copier,
        }
    }
}

impl JobContainerPreparerPort for PrepareJobContainerService {
    fn prepare(
        &self,
        request: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError> {
        let image = request.container_image().unwrap_or(DEFAULT_CONTAINER_IMAGE);
        let image = self
            .image_puller
            .pull(PullJobImageRequest::new(image.to_string()))
            .map_err(|error| PrepareJobContainerError::Image(error.to_string()))?;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or(0);
        let container_name = format!(
            "ephact-{}-{}-{}",
            request.job_id(),
            process::id(),
            timestamp
        );
        let legacy_container_name = format!("ephact-{}", request.job_id());
        let container = self
            .container_creator
            .create(CreateJobContainerRequest::new(
                image.clone(),
                container_name.clone(),
                legacy_container_name,
                request.repo_path().to_path_buf(),
                request.allow_repo_writes(),
            ))
            .map_err(|error| PrepareJobContainerError::Container(error.to_string()))?;

        if !request.allow_repo_writes() {
            self.repository_copier
                .copy(
                    CopyRepositoryToContainerRequest::new(
                        request.repo_path().to_path_buf(),
                        "/workspace".to_string(),
                    ),
                    container.as_ref(),
                )
                .map_err(|error| PrepareJobContainerError::Repository(error.to_string()))?;
        }

        Ok(PreparedJobContainerResponse::new(container, container_name))
    }
}
