use crate::{
    application::ports::outbound::prepare_job_container_port::PrepareJobContainerPort,
    infrastructure::containers::{
        create_job_container_port::CreateJobContainerPort, pull_job_image_port::PullJobImagePort,
    },
};
use std::{error::Error, process};

use crate::application::dtos::requests::CreateJobContainerRequest;
use crate::application::dtos::requests::PrepareJobContainerRequest;
use crate::application::dtos::requests::PullJobImageRequest;
use crate::application::dtos::responses::PreparedJobContainerResponse;

/// Service that prepares a job's container: pulls the image the job needs and
/// creates the container its steps run in.
pub struct PrepareJobContainerService {
    image_puller: Box<dyn PullJobImagePort>,
    container_creator: Box<dyn CreateJobContainerPort>,
}

impl PrepareJobContainerService {
    pub fn new(
        image_puller: Box<dyn PullJobImagePort>,
        container_creator: Box<dyn CreateJobContainerPort>,
    ) -> Self {
        Self {
            image_puller,
            container_creator,
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
            ))?;

        Ok(PreparedJobContainerResponse::new(container, container_name))
    }
}
