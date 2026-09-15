#![allow(dead_code)]
use std::collections::HashMap;

use ephact::application::dtos::responses::ExecResultResponse;
use ephact::application::dtos::responses::FileEntryResponse;
use ephact::application::dtos::responses::RunnerContextResponse;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

/// Container that succeeds at everything without recording anything, for tests
/// that need a container handle but never inspect it.
pub struct StubContainer;

impl ContainerPort for StubContainer {
    fn exec(
        &self,
        _cmd: &[String],
        _workdir: Option<&str>,
        _env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        Ok(ExecResultResponse::new(0, String::new(), String::new()))
    }

    fn copy_to(&self, _path: &str, _entries: &[FileEntryResponse]) -> Result<(), ContainerError> {
        Ok(())
    }

    fn copy_from(&self, _path: &str) -> Result<Vec<FileEntryResponse>, ContainerError> {
        Ok(vec![])
    }

    fn remove(&self) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError> {
        Ok(RunnerContextResponse::default())
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
