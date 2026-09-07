use std::collections::HashMap;

use crate::application::dtos::ExecResult;
use crate::application::dtos::FileEntry;
use crate::application::dtos::RunnerContext;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::ContainerError;
use crate::domain::events::OutputStream;
use crate::infrastructure::containers::bollard_wrapper::Client;
use crate::infrastructure::containers::bollard_wrapper::types::RemoveContainerOptions;
use crate::infrastructure::containers::exec_streaming_support::{
    exec_options, run_streaming_exec, runner_context_with_container_env,
};
use crate::infrastructure::containers::tar_transfer::{
    download_archive, pack_entries, unpack_entries, upload_archive,
};

/// A running Podman container, created by [`PodmanRuntime`].
pub(super) struct PodmanContainer {
    pub(super) client: Client,
    pub(super) container_id: String,
    pub(super) runtime: tokio::runtime::Handle,
    pub(super) runner_context: RunnerContext,
}

impl ContainerPort for PodmanContainer {
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
            &self.client,
            &self.container_id,
            options,
            on_output,
        ))
    }

    fn copy_to(&self, container_path: &str, entries: &[FileEntry]) -> Result<(), ContainerError> {
        let archive = pack_entries(entries, &self.container_id)?;
        self.runtime.block_on(upload_archive(
            &self.client,
            &self.container_id,
            container_path,
            archive,
        ))
    }

    fn copy_from(&self, container_path: &str) -> Result<Vec<FileEntry>, ContainerError> {
        self.runtime.block_on(async {
            let archive =
                download_archive(&self.client, &self.container_id, container_path).await?;
            unpack_entries(&archive, &self.container_id)
        })
    }

    fn remove(&self) -> Result<(), ContainerError> {
        self.runtime.block_on(async {
            self.client
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
                .client
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
