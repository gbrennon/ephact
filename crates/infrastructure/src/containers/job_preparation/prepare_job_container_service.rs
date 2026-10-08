use std::{collections::HashMap, process, time::SystemTime};

use crate::{
    application::{
        dtos::{
            requests::{
                CopyRepositoryToContainerRequest, CreateJobContainerRequest,
                PrepareJobContainerRequest, PullJobImageRequest,
            },
            responses::PreparedJobContainerResponse,
        },
        errors::PrepareJobContainerError,
        ports::outbound::{
            ContainerPort, CopyRepositoryToContainerPort, CreateJobContainerPort,
            JobContainerPreparerPort, PullJobImagePort,
        },
    },
    containers::workspace::{RUNNER_ENV_FILE, RUNNER_OUTPUT_FILE, RUNNER_PATH_FILE},
};

const DEFAULT_CONTAINER_IMAGE: &str = "ubuntu:24.04";

/// Runner-compatible default image for jobs without an explicit container image.
pub const GITHUB_HOSTED_RUNNER_IMAGE: &str = "ghcr.io/catthehacker/ubuntu:act-24.04";

/// Prepares isolated job containers from generic container image requests.
pub struct PrepareJobContainerService {
    default_image: String,
    image_puller: Box<dyn PullJobImagePort>,
    container_creator: Box<dyn CreateJobContainerPort>,
    repository_copier: Box<dyn CopyRepositoryToContainerPort>,
}

impl PrepareJobContainerService {
    /// Creates a service with the generic Ubuntu container image as its fallback.
    pub fn new(
        image_puller: Box<dyn PullJobImagePort>,
        container_creator: Box<dyn CreateJobContainerPort>,
        repository_copier: Box<dyn CopyRepositoryToContainerPort>,
    ) -> Self {
        Self::with_default_image(
            DEFAULT_CONTAINER_IMAGE,
            image_puller,
            container_creator,
            repository_copier,
        )
    }

    /// Creates a service with an explicit fallback image for jobs without one.
    pub fn with_default_image(
        default_image: impl Into<String>,
        image_puller: Box<dyn PullJobImagePort>,
        container_creator: Box<dyn CreateJobContainerPort>,
        repository_copier: Box<dyn CopyRepositoryToContainerPort>,
    ) -> Self {
        Self {
            default_image: default_image.into(),
            image_puller,
            container_creator,
            repository_copier,
        }
    }

    fn initialize_runner_command_files(
        &self,
        container: &dyn ContainerPort,
    ) -> Result<(), PrepareJobContainerError> {
        let command = format!(
            "mkdir -p /tmp && touch {RUNNER_ENV_FILE} {RUNNER_OUTPUT_FILE} {RUNNER_PATH_FILE}"
        );
        let result = container
            .exec(&["sh".into(), "-c".into(), command], None, &HashMap::new())
            .map_err(|error| PrepareJobContainerError::Container(format!("{error:?}")))?;
        if result.exit_code() == 0 {
            return Ok(());
        }
        Err(PrepareJobContainerError::Container(format!(
            "failed to initialize runner command files: {}",
            result.stderr()
        )))
    }

    fn copy_repository_if_needed(
        &self,
        request: &PrepareJobContainerRequest,
        container: &dyn ContainerPort,
    ) -> Result<(), PrepareJobContainerError> {
        if request.allow_repo_writes() {
            return Ok(());
        }
        self.repository_copier
            .copy(
                CopyRepositoryToContainerRequest::new(
                    request.repo_path().to_path_buf(),
                    "/workspace".to_string(),
                ),
                container,
            )
            .map_err(|error| PrepareJobContainerError::Repository(error.to_string()))
    }
}

impl JobContainerPreparerPort for PrepareJobContainerService {
    fn prepare(
        &self,
        request: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError> {
        let image = request
            .container_image()
            .unwrap_or(&self.default_image)
            .to_string();
        let image = self
            .image_puller
            .pull(PullJobImageRequest::new(image))
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
        self.initialize_runner_command_files(container.as_ref())?;

        self.copy_repository_if_needed(&request, container.as_ref())?;

        Ok(PreparedJobContainerResponse::new(container, container_name))
    }
}
