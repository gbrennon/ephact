use futures_util::StreamExt;
use tokio::runtime::Runtime;

use super::bollard_wrapper::types::{
    ContainerCreateBody, CreateContainerOptionsBuilder, CreateImageOptionsBuilder, HostConfig,
    InspectContainerOptions, KillContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use super::bollard_wrapper::{API_DEFAULT_VERSION, AuthCredentials, Client};
use super::podman_container::PodmanContainer;
use crate::{
    application::{
        dtos::{ContainerConfig, HostInfo},
        ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
    },
    domain::errors::ContainerError,
};

/// Podman-based container runtime adapter using the bollard crate.
///
/// Podman exposes a Docker-compatible API. This adapter connects via the
/// Podman socket, trying rootless first (`/run/user/$UID/podman/podman.sock`)
/// then falling back to the root socket (`/run/podman/podman.sock`).
pub struct PodmanRuntime {
    client: Client,
    runtime: Runtime,
}

impl PodmanRuntime {
    /// Create a new Podman runtime adapter.
    ///
    /// Probes the rootless socket first, then the root socket. Returns
    /// `ContainerError::NotAvailable` if neither is reachable.
    pub fn new() -> Result<Self, ContainerError> {
        let runtime = Runtime::new().map_err(|e| ContainerError::Internal(e.to_string()))?;

        let uid = unsafe { libc::getuid() };
        let rootless_socket = format!("unix:///run/user/{}/podman/podman.sock", uid);
        let root_socket = "unix:///run/podman/podman.sock";

        let client = Client::connect_with_unix(&rootless_socket, 120, API_DEFAULT_VERSION)
            .or_else(|_| Client::connect_with_unix(root_socket, 120, API_DEFAULT_VERSION))
            .map_err(|_| ContainerError::NotAvailable)?;

        Ok(Self { client, runtime })
    }
}

impl ContainerRuntimePort for PodmanRuntime {
    fn pull_image(&self, image: &str, platform: Option<&str>) -> Result<(), ContainerError> {
        let mut options_builder = CreateImageOptionsBuilder::new().from_image(image);
        if let Some(p) = platform {
            options_builder = options_builder.platform(p);
        }
        let options = options_builder.build();

        self.runtime.block_on(async {
            let mut stream = self
                .client
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
        config: &ContainerConfig,
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
            self.client
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
            self.client
                .start_container(&container.id, None::<StartContainerOptions>)
                .await
                .map_err(|e| {
                    ContainerError::CreationFailed(
                        config.name().unwrap_or_default().to_string().to_string(),
                        e.to_string(),
                    )
                })
        })?;

        Ok(Box::new(PodmanContainer::new(
            self.client.clone(),
            container.id,
            self.runtime.handle().clone(),
            config.runner_context().clone(),
        )))
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
            let force = match self
                .client
                .inspect_container(name, None::<InspectContainerOptions>)
                .await
            {
                Ok(inspect) => inspect.state.and_then(|s| s.running).unwrap_or(false),
                Err(_) => return Ok(()),
            };
            self.client
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
                .client
                .inspect_container(name, None::<InspectContainerOptions>)
                .await
                && !inspect.state.and_then(|s| s.running).unwrap_or(false)
            {
                return Ok(());
            }
            self.client
                .stop_container(name, None)
                .await
                .map_err(|e| ContainerError::RemovalFailed(name.to_string(), e.to_string()))
        })
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
            match self
                .client
                .inspect_container(name, None::<InspectContainerOptions>)
                .await
            {
                Ok(inspect) if inspect.state.as_ref().and_then(|s| s.running) == Some(true) => {}
                _ => return Ok(()),
            }
            self.client
                .kill_container(name, None::<KillContainerOptions>)
                .await
                .map_err(|e| ContainerError::KillFailed(name.to_string(), e.to_string()))
        })
    }

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        self.runtime.block_on(async {
            let info = self
                .client
                .version()
                .await
                .map_err(|_| ContainerError::NotAvailable)?;

            Ok(HostInfo::new(
                info.os.unwrap_or_else(|| "linux".to_string()),
                info.arch.unwrap_or_else(|| "amd64".to_string()),
                info.version.unwrap_or_else(|| "unknown".to_string()),
            ))
        })
    }
}
