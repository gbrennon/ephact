use futures_util::StreamExt;
use tokio::runtime::Runtime;

use super::bollard_wrapper::types::{
    ContainerCreateBody, CreateContainerOptionsBuilder, CreateImageOptionsBuilder, HostConfig,
    InspectContainerOptions, KillContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use super::bollard_wrapper::{AuthCredentials, Client};
use super::docker_container::DockerContainer;
use crate::application::dtos::responses::ContainerConfigResponse;
use crate::application::dtos::responses::HostInfoResponse;
use crate::application::ports::outbound::ContainerRuntimePort;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::ContainerError;

/// Docker-based container runtime adapter using the bollard crate.
///
/// Connects to the Docker daemon via the default Unix socket
/// (`/var/run/docker.sock`).
pub struct DockerRuntime {
    docker: Client,
    runtime: Runtime,
}

impl DockerRuntime {
    /// Create a new Docker runtime adapter connected to the local Docker daemon.
    pub fn new() -> Result<Self, ContainerError> {
        let docker =
            Client::connect_with_local_defaults().map_err(|_e| ContainerError::NotAvailable)?;
        let runtime = Runtime::new().map_err(|e| ContainerError::Internal(e.to_string()))?;
        Ok(Self { docker, runtime })
    }
}

impl ContainerRuntimePort for DockerRuntime {
    fn pull_image(&self, image: &str, platform: Option<&str>) -> Result<(), ContainerError> {
        let mut options_builder = CreateImageOptionsBuilder::new().from_image(image);
        if let Some(p) = platform {
            options_builder = options_builder.platform(p);
        }
        let options = options_builder.build();
        self.runtime.block_on(async {
            let mut stream = self
                .docker
                .create_image(Some(options), None, None::<AuthCredentials>);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(_info) => {}
                    Err(e) => {
                        return Err(ContainerError::ImagePullFailed(
                            image.to_string(),
                            e.to_string(),
                        ));
                    }
                }
            }
            Ok(())
        })
    }

    fn create_container(
        &self,
        config: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        let env_list: Vec<String> = config
            .env()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        let host_config = HostConfig {
            binds: Some(config.binds().to_vec()),
            network_mode: config.network().map(str::to_string),
            ..Default::default()
        };

        let create_options = CreateContainerOptionsBuilder::new()
            .name(config.name().unwrap_or(""))
            .platform(config.platform().unwrap_or(""))
            .build();

        let container_config = ContainerCreateBody {
            image: Some(config.image().to_string()),
            env: Some(env_list),
            cmd: config.cmd().map(<[String]>::to_vec),
            entrypoint: config.entrypoint().map(<[String]>::to_vec),
            working_dir: config.workdir().map(str::to_string),
            host_config: Some(host_config),
            ..Default::default()
        };

        let container = self.runtime.block_on(async {
            self.docker
                .create_container(Some(create_options), container_config)
                .await
                .map_err(|e| {
                    ContainerError::CreationFailed(
                        config.name().unwrap_or_default().to_string().to_string(),
                        e.to_string(),
                    )
                })
        })?;

        self.runtime.block_on(async {
            self.docker
                .start_container(&container.id, None::<StartContainerOptions>)
                .await
                .map_err(|e| {
                    ContainerError::CreationFailed(
                        config.name().unwrap_or_default().to_string().to_string(),
                        e.to_string(),
                    )
                })
        })?;

        Ok(Box::new(DockerContainer::new(
            self.docker.clone(),
            container.id,
            self.runtime.handle().clone(),
            config.runner_context().clone(),
        )))
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
            let force = match self
                .docker
                .inspect_container(name, None::<InspectContainerOptions>)
                .await
            {
                Ok(inspect) => inspect.state.and_then(|s| s.running).unwrap_or(false),
                Err(_) => return Ok(()),
            };
            self.docker
                .remove_container(
                    name,
                    Some(RemoveContainerOptions {
                        force,
                        ..Default::default()
                    }),
                )
                .await
                .map_err(|e| ContainerError::RemovalFailed(name.to_string(), e.to_string()))
        })
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
            if let Ok(inspect) = self
                .docker
                .inspect_container(name, None::<InspectContainerOptions>)
                .await
                && !inspect.state.and_then(|s| s.running).unwrap_or(false)
            {
                return Ok(());
            }
            self.docker
                .stop_container(name, None)
                .await
                .map_err(|e| ContainerError::RemovalFailed(name.to_string(), e.to_string()))
        })
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
            match self
                .docker
                .inspect_container(name, None::<InspectContainerOptions>)
                .await
            {
                Ok(inspect) if inspect.state.as_ref().and_then(|s| s.running) == Some(true) => {}
                _ => return Ok(()),
            }
            self.docker
                .kill_container(name, None::<KillContainerOptions>)
                .await
                .map_err(|e| ContainerError::KillFailed(name.to_string(), e.to_string()))
        })
    }

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        self.runtime.block_on(async {
            let info = self
                .docker
                .version()
                .await
                .map_err(|_e| ContainerError::NotAvailable)?;

            Ok(HostInfoResponse::new(
                info.os.unwrap_or_else(|| "linux".to_string()),
                info.arch.unwrap_or_else(|| "amd64".to_string()),
                info.version.unwrap_or_else(|| "unknown".to_string()),
            ))
        })
    }
}
