#![allow(dead_code)]
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

use ephact::application::dtos::responses::ExecResultResponse;
use ephact::application::dtos::responses::FileEntryResponse;
use ephact::application::dtos::responses::RunnerContextResponse;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

/// Container that succeeds at everything and records what it was asked to run
/// and copy. Every clone observes the same recording.
#[derive(Clone, Default)]
pub struct StubRecordingContainer {
    executed_commands: Arc<Mutex<Vec<Vec<String>>>>,
    exec_environments: Arc<Mutex<Vec<HashMap<String, String>>>>,
    copied_paths: Arc<Mutex<Vec<String>>>,
    copied_files: Arc<Mutex<Vec<Vec<FileEntryResponse>>>>,
}

impl StubRecordingContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn executed_commands(&self) -> Vec<Vec<String>> {
        self.executed_commands.lock().clone()
    }

    pub fn exec_environments(&self) -> Vec<HashMap<String, String>> {
        self.exec_environments.lock().clone()
    }

    pub fn copied_paths(&self) -> Vec<String> {
        self.copied_paths.lock().clone()
    }

    pub fn copied_files(&self) -> Vec<Vec<FileEntryResponse>> {
        self.copied_files.lock().clone()
    }
}

impl ContainerPort for StubRecordingContainer {
    fn exec(
        &self,
        cmd: &[String],
        _workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        self.executed_commands.lock().push(cmd.to_vec());
        self.exec_environments.lock().push(env.clone());
        Ok(ExecResultResponse::new(0, String::new(), String::new()))
    }

    fn copy_to(&self, path: &str, entries: &[FileEntryResponse]) -> Result<(), ContainerError> {
        self.copied_paths.lock().push(path.to_string());
        self.copied_files.lock().push(entries.to_vec());
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
}
