#![allow(dead_code)]
use std::collections::HashMap;

use ephact::application::dtos::responses::ExecResultResponse;
use ephact::application::dtos::responses::FileEntryResponse;
use ephact::application::dtos::responses::RunnerContextResponse;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

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
}
