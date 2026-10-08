use std::{collections::HashMap, error::Error, sync::Arc};

use crate::{
    application::{
        dtos::{
            requests::CreateJobContainerRequest,
            responses::{ContainerConfigOptions, ContainerConfigResponse, RunnerContextResponse},
        },
        ports::outbound::{ContainerPort, ContainerRuntimePort, CreateJobContainerPort},
    },
    containers::{
        job_preparation::host_ssh_forwarding_config::HostSshForwardingConfig,
        workspace::CONTAINER_WORKSPACE,
    },
};

/// Service that creates the container a job's steps run in, removing any
/// container left behind by an earlier run of the same job first.
pub struct CreateJobContainerService {
    runtime: Arc<dyn ContainerRuntimePort>,
    ssh_forwarding: HostSshForwardingConfig,
}

impl CreateJobContainerService {
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self::with_ssh_forwarding(runtime, HostSshForwardingConfig::disabled())
    }

    pub fn with_ssh_forwarding(
        runtime: Arc<dyn ContainerRuntimePort>,
        ssh_forwarding: HostSshForwardingConfig,
    ) -> Self {
        Self {
            runtime,
            ssh_forwarding,
        }
    }

    fn host_ssh_mounts(&self) -> Result<(Vec<String>, HashMap<String, String>), Box<dyn Error>> {
        let Some(socket_path) = self
            .ssh_forwarding
            .socket_path()
            .map_err(Box::<dyn Error>::from)?
        else {
            return Ok((Vec::new(), HashMap::new()));
        };
        let container_socket = "/tmp/ephact-ssh-agent.sock";
        let bind = format!("{}:{container_socket}", socket_path.display());
        let environment =
            HashMap::from([("SSH_AUTH_SOCK".to_string(), container_socket.to_string())]);
        Ok((vec![bind], environment))
    }
}

impl CreateJobContainerPort for CreateJobContainerService {
    fn create(
        &self,
        request: CreateJobContainerRequest,
    ) -> Result<Box<dyn ContainerPort>, Box<dyn Error>> {
        let _ = self
            .runtime
            .remove_container(request.legacy_container_name());
        let _ = self.runtime.remove_container(request.container_name());

        let (mut binds, environment) = self.host_ssh_mounts()?;
        if request.allow_repo_writes() {
            binds.push(format!(
                "{}:{}:Z",
                request.repo_path().display(),
                CONTAINER_WORKSPACE
            ));
        }
        let container_config = ContainerConfigResponse::new(
            request.image().to_string(),
            ContainerConfigOptions::default()
                .with_env(environment)
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
