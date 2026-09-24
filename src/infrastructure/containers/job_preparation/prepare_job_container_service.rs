use std::{process, time::SystemTime};

use super::{
    copy_repository_to_container_port::CopyRepositoryToContainerPort,
    create_job_container_port::CreateJobContainerPort,
    pull_job_image_port::PullJobImagePort,
};
use crate::application::{
    dtos::{
        requests::{
            CopyRepositoryToContainerRequest, CreateJobContainerRequest,
            PrepareJobContainerRequest, PullJobImageRequest,
        },
        responses::PreparedJobContainerResponse,
    },
    errors::PrepareJobContainerError,
    ports::outbound::job_container_preparer_port::JobContainerPreparerPort,
};

pub struct PrepareJobContainerService {
    image_puller: Box<dyn PullJobImagePort>,
    container_creator: Box<dyn CreateJobContainerPort>,
    repository_copier: Box<dyn CopyRepositoryToContainerPort>,
}

impl PrepareJobContainerService {
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
        let image = self
            .image_puller
            .execute(PullJobImageRequest::new(
                request.runs_on().map(str::to_string),
            ))
            .map_err(|error| PrepareJobContainerError::Image(error.to_string()))?;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or(0);
        let container_name = format!(
            "ephemeral-act-{}-{}-{}",
            request.job_id(),
            process::id(),
            timestamp
        );
        let legacy_container_name = format!("ephemeral-act-{}", request.job_id());
        let container = self
            .container_creator
            .execute(CreateJobContainerRequest::new(
                image.clone(),
                container_name.clone(),
                legacy_container_name,
                request.repo_path().to_path_buf(),
                request.allow_repo_writes(),
            ))
            .map_err(|error| PrepareJobContainerError::Container(error.to_string()))?;

        if !request.allow_repo_writes() {
            self.repository_copier
                .execute(
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
