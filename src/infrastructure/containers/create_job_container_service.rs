use super::{create_job_container_port::CreateJobContainerPort, workspace::CONTAINER_WORKSPACE};
use std::{collections::HashMap, error::Error, sync::Arc};

use crate::application::dtos::requests::CreateJobContainerRequest;
use crate::application::dtos::responses::ContainerConfigResponse;
use crate::application::dtos::responses::RunnerContextResponse;
use crate::application::ports::outbound::ContainerRuntimePort;
use crate::application::ports::outbound::container_port::ContainerPort;

/// Service that creates the container a job's steps run in, removing any
/// container left behind by an earlier run of the same job first.
pub struct CreateJobContainerService {
    runtime: Arc<dyn ContainerRuntimePort>,
}

impl CreateJobContainerService {
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self { runtime }
    }
}

impl CreateJobContainerPort for CreateJobContainerService {
    fn execute(
        &self,
        request: CreateJobContainerRequest<'_>,
    ) -> Result<Box<dyn ContainerPort>, Box<dyn Error>> {
        let _ = self
            .runtime
            .remove_container(request.legacy_container_name());
        let _ = self.runtime.remove_container(request.container_name());

        let bind_suffix = if request.allow_repo_writes() {
            ":Z"
        } else {
            ":ro,Z"
        };
        let container_config = ContainerConfigResponse::new(
            request.image().to_string(),
            None,
            HashMap::new(),
            vec![format!(
                "{}:{}{}",
                request.repo_path().display(),
                CONTAINER_WORKSPACE,
                bind_suffix
            )],
            Some(CONTAINER_WORKSPACE.into()),
            Some(vec!["sleep".into(), "infinity".into()]),
            None,
            None,
            Some(request.container_name().to_string()),
            RunnerContextResponse::default(),
        );

        self.runtime
            .create_container(&container_config)
            .map_err(|e| -> Box<dyn Error> { format!("{:?}", e).into() })
    }
}
