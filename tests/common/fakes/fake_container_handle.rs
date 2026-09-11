#![allow(dead_code)]
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

use ephact::application::dtos::responses::ExecResultResponse;
use ephact::application::dtos::responses::FileEntryResponse;
use ephact::application::dtos::responses::RunnerContextResponse;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

/// Container handle a [`super::fake_runtime::FakeRuntime`] creates.
///
/// Records what it was asked to execute and copy, and answers each execution
/// with the next queued result, or with a successful empty result once the
/// queue is drained. Reading a file the runner writes (`cat`) fails, the way a
/// container that never wrote the file behaves.
pub struct FakeContainerHandle {
    exec_results: Arc<Mutex<Vec<ExecResultResponse>>>,
    executed_commands: Arc<Mutex<Vec<Vec<String>>>>,
    exec_environments: Arc<Mutex<Vec<HashMap<String, String>>>>,
    copied_paths: Arc<Mutex<Vec<String>>>,
}

impl FakeContainerHandle {
    pub fn new(
        exec_results: Arc<Mutex<Vec<ExecResultResponse>>>,
        executed_commands: Arc<Mutex<Vec<Vec<String>>>>,
        exec_environments: Arc<Mutex<Vec<HashMap<String, String>>>>,
        copied_paths: Arc<Mutex<Vec<String>>>,
    ) -> Self {
        Self {
            exec_results,
            executed_commands,
            exec_environments,
            copied_paths,
        }
    }
}

impl ContainerPort for FakeContainerHandle {
    fn exec(
        &self,
        cmd: &[String],
        _workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        if cmd.first().map(String::as_str) == Some("cat") {
            return Err(ContainerError::ExecutionFailed(
                "fake".into(),
                "No such file or directory".into(),
            ));
        }

        self.executed_commands.lock().push(cmd.to_vec());
        self.exec_environments.lock().push(env.clone());

        let mut results = self.exec_results.lock();
        if results.is_empty() {
            Ok(ExecResultResponse::new(0, String::new(), String::new()))
        } else {
            Ok(results.remove(0))
        }
    }

    fn copy_to(&self, path: &str, _entries: &[FileEntryResponse]) -> Result<(), ContainerError> {
        self.copied_paths.lock().push(path.to_string());
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
