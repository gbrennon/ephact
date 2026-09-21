#![allow(dead_code)]
use std::collections::HashMap;

use ephact::{
    application::{
        dtos::responses::{ExecResultResponse, FileEntryResponse, RunnerContextResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::ContainerError,
};

/// Container that answers every execution with one prepared result.
pub struct StubScriptedContainer {
    exit_code: i64,
    stdout: String,
}

impl StubScriptedContainer {
    pub fn answering(exit_code: i64, stdout: &str) -> Self {
        Self {
            exit_code,
            stdout: stdout.to_string(),
        }
    }
}

impl ContainerPort for StubScriptedContainer {
    fn exec(
        &self,
        _cmd: &[String],
        _workdir: Option<&str>,
        _env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        Ok(ExecResultResponse::new(
            self.exit_code,
            self.stdout.clone(),
            String::new(),
        ))
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
