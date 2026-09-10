use super::{create_job_container_port::CreateJobContainerPort, workspace::CONTAINER_WORKSPACE};
use std::{collections::HashMap, error::Error, sync::Arc};

use crate::application::{
    dtos::{ContainerConfig, CreateJobContainerRequest, RunnerContext},
    ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
};

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
    ) -> Result<Arc<dyn ContainerPort>, Box<dyn Error>> {
        let _ = self
            .runtime
            .remove_container(request.legacy_container_name());
        let _ = self.runtime.remove_container(request.container_name());

        let container_config = ContainerConfig::new(
            request.image().to_string(),
            None,
            HashMap::new(),
            vec![format!(
                "{}:{}:Z",
                request.repo_path().display(),
                CONTAINER_WORKSPACE
            )],
            Some(CONTAINER_WORKSPACE.into()),
            Some(vec!["sleep".into(), "infinity".into()]),
            None,
            None,
            Some(request.container_name().to_string()),
            RunnerContext::default(),
        );

        Ok(Arc::from(
            self.runtime
                .create_container(&container_config)
                .map_err(|e| format!("{:?}", e))?,
        ))
    }
}
