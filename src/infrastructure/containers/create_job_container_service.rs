use super::{create_job_container_port::CreateJobContainerPort, workspace::CONTAINER_WORKSPACE};
use std::{collections::HashMap, error::Error, sync::Arc};

use crate::application::dtos::requests::CreateJobContainerRequest;
use crate::application::dtos::responses::{
    ContainerConfigOptions, ContainerConfigResponse, RunnerContextResponse,
};
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
        request: CreateJobContainerRequest,
    ) -> Result<Box<dyn ContainerPort>, Box<dyn Error>> {
        let _ = self
            .runtime
            .remove_container(request.legacy_container_name());
        let _ = self.runtime.remove_container(request.container_name());

        let binds = if request.allow_repo_writes() {
            vec![format!(
                "{}:{}:Z",
                request.repo_path().display(),
                CONTAINER_WORKSPACE
            )]
        } else {
            vec![]
        };
        let container_config = ContainerConfigResponse::new(
            request.image().to_string(),
            ContainerConfigOptions::default()
                .with_env(HashMap::new())
                .with_binds(binds)
                .with_workdir(Some(CONTAINER_WORKSPACE.into()))
                .with_cmd(Some(vec!["sleep".into(), "infinity".into()]))
                .with_name(Some(request.container_name().to_string()))
                .with_runner_context(RunnerContextResponse::default()),
        );

        self.runtime
            .create_container(&container_config)
            .map_err(|e| -> Box<dyn Error> { format!("{:?}", e).into() })
    }
}
