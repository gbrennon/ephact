use std::collections::HashMap;

use crate::application::dtos::responses::ExecResultResponse;
use crate::application::dtos::responses::FileEntryResponse;
use crate::application::dtos::responses::RunnerContextResponse;
use crate::domain::errors::ContainerError;
use crate::domain::events::OutputStream;

/// Outbound port for working inside one running container.
///
/// The application layer coordinates a run in terms of "execute this command"
/// and "move these files"; which runtime backs the container is an
/// infrastructure concern behind this port.
pub trait ContainerPort: Send + Sync {
    /// Executes a command inside the container.
    fn exec(
        &self,
        cmd: &[String],
        workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError>;

    /// Executes a command inside the container, forwarding each output chunk
    /// to `on_output` as it is produced.
    ///
    /// The returned [`ExecResultResponse`] carries the same output accumulated by the
    /// sink. Implementations that cannot stream may buffer and deliver the
    /// whole output once, right before returning.
    fn exec_streaming(
        &self,
        cmd: &[String],
        workdir: Option<&str>,
        env: &HashMap<String, String>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        let result = self.exec(cmd, workdir, env)?;
        if !result.stdout().is_empty() {
            on_output(OutputStream::StandardOutput, result.stdout());
        }
        if !result.stderr().is_empty() {
            on_output(OutputStream::StandardError, result.stderr());
        }
        Ok(result)
    }

    /// Copies the given entries into the container at `container_path`.
    fn copy_to(
        &self,
        container_path: &str,
        entries: &[FileEntryResponse],
    ) -> Result<(), ContainerError>;

    /// Reads the entries stored under `container_path` out of the container.
    fn copy_from(&self, container_path: &str) -> Result<Vec<FileEntryResponse>, ContainerError>;

    /// Removes the container.
    fn remove(&self) -> Result<(), ContainerError>;

    /// Reports the runner context the container exposes to steps.
    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError>;
}
