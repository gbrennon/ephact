use std::collections::HashMap;

use crate::application::dtos::ExecResult;
use crate::application::dtos::FileEntry;
use crate::application::dtos::RunnerContext;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::ContainerError;
use crate::domain::events::OutputStream;
use super::bollard_wrapper::Client;
use super::bollard_wrapper::types::RemoveContainerOptions;
use super::exec_streaming_support::{
    exec_options, run_streaming_exec, runner_context_with_container_env,
};
use super::tar_transfer::{
    download_archive, pack_entries, unpack_entries, upload_archive,
};

/// A running Docker container, created by [`DockerRuntime`].
pub(super) struct DockerContainer {
    docker: Client,
    container_id: String,
    runtime: tokio::runtime::Handle,
    runner_context: RunnerContext,
}

impl DockerContainer {
    pub(super) fn new(
        docker: Client,
        container_id: String,
        runtime: tokio::runtime::Handle,
        runner_context: RunnerContext,
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
    ) -> Result<ExecResult, ContainerError> {
        self.exec_streaming(cmd, workdir, env, &mut |_, _| {})
    }

    fn exec_streaming(
        &self,
        cmd: &[String],
        workdir: Option<&str>,
        env: &HashMap<String, String>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResult, ContainerError> {
        let options = exec_options(cmd, workdir, env);
        self.runtime.block_on(run_streaming_exec(
            &self.docker,
            &self.container_id,
            options,
            on_output,
        ))
    }

    fn copy_to(&self, container_path: &str, entries: &[FileEntry]) -> Result<(), ContainerError> {
        let archive = pack_entries(entries, &self.container_id)?;
        self.runtime.block_on(upload_archive(
            &self.docker,
            &self.container_id,
            container_path,
            archive,
        ))
    }

    fn copy_from(&self, container_path: &str) -> Result<Vec<FileEntry>, ContainerError> {
        self.runtime.block_on(async {
            let archive =
                download_archive(&self.docker, &self.container_id, container_path).await?;
            unpack_entries(&archive, &self.container_id)
        })
    }

    fn remove(&self) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
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
    fn get_runner_context(&self) -> Result<RunnerContext, ContainerError> {
        self.runtime.block_on(async {
            let info = self
                .docker
                .inspect_container(&self.container_id, None)
                .await
                .map_err(|_e| ContainerError::NotFound(self.container_id.clone()))?;

            let container_env = info
                .config
                .and_then(|config| config.env)
                .unwrap_or_default();
            Ok(runner_context_with_container_env(
                &self.runner_context,
                container_env,
            ))
        })
    }
}
