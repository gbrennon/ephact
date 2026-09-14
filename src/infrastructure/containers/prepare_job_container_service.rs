use crate::{
    application::dtos::requests::CopyRepositoryToContainerRequest,
    application::ports::outbound::prepare_job_container_port::PrepareJobContainerPort,
    infrastructure::containers::{
        copy_repository_to_container_port::CopyRepositoryToContainerPort,
        create_job_container_port::CreateJobContainerPort, pull_job_image_port::PullJobImagePort,
    },
};
use std::{error::Error, process};

use crate::application::dtos::requests::CreateJobContainerRequest;
use crate::application::dtos::requests::PrepareJobContainerRequest;
use crate::application::dtos::requests::PullJobImageRequest;
use crate::application::dtos::responses::PreparedJobContainerResponse;

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
impl PrepareJobContainerPort for PrepareJobContainerService {
    fn execute(
        &self,
        request: PrepareJobContainerRequest<'_>,
    ) -> Result<PreparedJobContainerResponse, Box<dyn Error>> {
        let image = self
            .image_puller
            .execute(PullJobImageRequest::new(request.runs_on()))?;

        let container_name = format!("ephemeral-act-{}-{}", request.job_id(), process::id());
        let legacy_container_name = format!("ephemeral-act-{}", request.job_id());

        let container = self
            .container_creator
            .execute(CreateJobContainerRequest::new(
                &image,
                &container_name,
                &legacy_container_name,
                request.repo_path(),
                request.allow_repo_writes(),
            ))?;

        if !request.allow_repo_writes() {
            self.repository_copier.execute(
                CopyRepositoryToContainerRequest::new(request.repo_path(), "/workspace"),
                container.as_ref(),
            )?;
        }

        Ok(PreparedJobContainerResponse::new(container, container_name))
    }
}
