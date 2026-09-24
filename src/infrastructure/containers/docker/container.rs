use std::collections::HashMap;

use super::super::{
    bollard_wrapper::{Client, types::RemoveContainerOptions},
    runtime::block_on_runtime::RuntimeBlocker,
    streaming::{
        exec_streaming_support::ExecStreamingSupport, runner_context_env::RunnerContextEnv,
        tar_transfer::TarTransfer,
    },
};
use crate::{
    application::{
        dtos::responses::{ExecResultResponse, RunnerContextResponse},
        ports::outbound::container_port::{ContainerPort, ExecOptions},
    },
    domain::{entities::FileEntry, errors::ContainerError, messages::events::OutputStream},
};

/// A running Docker container, created by [`DockerRuntime`].
pub(super) struct DockerContainer {
    docker: Client,
    container_id: String,
    runtime: tokio::runtime::Handle,
    runner_context: RunnerContextResponse,
}

impl DockerContainer {
    pub(super) fn new(
        docker: Client,
        container_id: String,
        runtime: tokio::runtime::Handle,
        runner_context: RunnerContextResponse,
    ) -> Self {
        Self {
            docker,
            container_id,
            runtime,
            runner_context,
        }
    }
}

impl ContainerPort for DockerContainer {
    fn exec(
        &self,
        cmd: &[String],
        workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        self.exec_streaming(ExecOptions::new(cmd, workdir, env), &mut |_, _| {})
    }

    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        let exec = ExecStreamingSupport::new(self.docker.clone(), self.container_id.clone());
        RuntimeBlocker::new(&self.runtime).block_on(exec.run(options, on_output))
    }

    fn copy_to(&self, container_path: &str, entries: &[FileEntry]) -> Result<(), ContainerError> {
        let transfer = TarTransfer::new(self.docker.clone(), self.container_id.clone());
        let archive = transfer.pack(entries)?;
        RuntimeBlocker::new(&self.runtime).block_on(transfer.upload(container_path, archive))
    }

    fn copy_from(&self, container_path: &str) -> Result<Vec<FileEntry>, ContainerError> {
        let transfer = TarTransfer::new(self.docker.clone(), self.container_id.clone());
        RuntimeBlocker::new(&self.runtime).block_on(async {
            let archive = transfer.download(container_path).await?;
            transfer.unpack(&archive)
        })
    }

    fn remove(&self) -> Result<(), ContainerError> {
        RuntimeBlocker::new(&self.runtime).block_on(async {
            self.docker
                .remove_container(
                    &self.container_id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await
                .map_err(|e| {
                    ContainerError::RemovalFailed(self.container_id.clone(), e.to_string())
                })
        })
    }

    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError> {
        RuntimeBlocker::new(&self.runtime).block_on(async {
            let info = self
                .docker
                .inspect_container(&self.container_id, None)
                .await
                .map_err(|_e| ContainerError::NotFound(self.container_id.clone()))?;

            let container_env = info
                .config
                .and_then(|config| config.env)
                .unwrap_or_default();
            Ok(
                RunnerContextEnv::new(self.runner_context.clone())
                    .with_container_env(container_env),
            )
        })
    }
}
