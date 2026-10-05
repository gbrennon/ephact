use std::collections::HashMap;

pub use crate::dtos::requests::ExecOptions;
use crate::{
    domain::{entities::FileEntry, errors::ContainerError, messages::events::OutputStream},
    dtos::responses::{ExecResultResponse, RunnerContextResponse},
};

/// Executes commands and transfers files within a running container.
pub trait ContainerPort: Send + Sync {
    /// Runs `cmd` in the optional working directory with `env` and returns its
    /// exit status, standard output, and standard error.
    ///
    /// # Errors
    ///
    /// Returns [`ContainerError`] when the command cannot be executed.
    fn exec(
        &self,
        cmd: &[String],
        workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError>;

    /// Runs the command described by [`ExecOptions`] and forwards each output
    /// chunk to `on_output` as it arrives.
    ///
    /// # Errors
    ///
    /// Returns [`ContainerError`] when streaming execution cannot be started or
    /// completed.
    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError>;

    /// Copies `entries` to `container_path`.
    ///
    /// # Errors
    ///
    /// Returns [`ContainerError`] when the transfer fails.
    fn copy_to(&self, container_path: &str, entries: &[FileEntry]) -> Result<(), ContainerError>;

    /// Reads file entries from `container_path`.
    ///
    /// # Errors
    ///
    /// Returns [`ContainerError`] when the transfer or unpacking fails.
    fn copy_from(&self, container_path: &str) -> Result<Vec<FileEntry>, ContainerError>;

    /// Removes this container.
    ///
    /// # Errors
    ///
    /// Returns [`ContainerError`] when removal fails.
    fn remove(&self) -> Result<(), ContainerError>;

    /// Returns runner context information, including the container environment.
    ///
    /// # Errors
    ///
    /// Returns [`ContainerError`] when the container cannot be inspected.
    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError>;
}
