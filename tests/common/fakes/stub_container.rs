#![allow(dead_code)]
use std::collections::HashMap;

use ephact::{
    application::{
        dtos::{ExecResult, FileEntry, RunnerContext},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::ContainerError,
};

/// Container that succeeds at everything without recording anything, for tests
/// that need a container handle but never inspect it.
pub struct StubContainer;

impl ContainerPort for StubContainer {
    fn exec(
        &self,
        _cmd: &[String],
        _workdir: Option<&str>,
        _env: &HashMap<String, String>,
    ) -> Result<ExecResult, ContainerError> {
        Ok(ExecResult::new(0, String::new(), String::new()))
    }

    fn copy_to(&self, _path: &str, _entries: &[FileEntry]) -> Result<(), ContainerError> {
        Ok(())
    }

    fn copy_from(&self, _path: &str) -> Result<Vec<FileEntry>, ContainerError> {
        Ok(vec![])
    }

    fn remove(&self) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get_runner_context(&self) -> Result<RunnerContext, ContainerError> {
        Ok(RunnerContext::default())
    }
}
