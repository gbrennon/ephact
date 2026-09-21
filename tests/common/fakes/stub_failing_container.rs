#![allow(dead_code)]
use std::collections::HashMap;

use ephact::{
    application::{
        dtos::responses::{ExecResultResponse, FileEntryResponse, RunnerContextResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::ContainerError,
};

/// Container that fails every operation, for tests that need to see how a
/// service surfaces a container failure.
pub struct StubFailingContainer;

impl ContainerPort for StubFailingContainer {
    fn exec(
        &self,
        _cmd: &[String],
        _workdir: Option<&str>,
        _env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        Err(ContainerError::ExecutionFailed(
            "stub".into(),
            "exec refused".into(),
        ))
    }

    fn copy_to(&self, _path: &str, _entries: &[FileEntryResponse]) -> Result<(), ContainerError> {
        Err(ContainerError::Internal("copy refused".into()))
    }

    fn copy_from(&self, _path: &str) -> Result<Vec<FileEntryResponse>, ContainerError> {
        Err(ContainerError::Internal("copy refused".into()))
    }

    fn remove(&self) -> Result<(), ContainerError> {
        Err(ContainerError::RemovalFailed(
            "stub".into(),
            "removal refused".into(),
        ))
    }

    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn exec_streaming(
        &self,
        options: ephact::application::ports::outbound::ExecOptions<'_>,
        on_output: &mut dyn FnMut(ephact::domain::messages::events::OutputStream, &str),
    ) -> Result<
        ephact::application::dtos::responses::ExecResultResponse,
        ephact::domain::errors::ContainerError,
    > {
        let result = self.exec(options.cmd(), options.workdir(), options.env())?;
        if !result.stdout().is_empty() {
            on_output(
                ephact::domain::messages::events::OutputStream::StandardOutput,
                result.stdout(),
            );
        }
        if !result.stderr().is_empty() {
            on_output(
                ephact::domain::messages::events::OutputStream::StandardError,
                result.stderr(),
            );
        }
        Ok(result)
    }
}
