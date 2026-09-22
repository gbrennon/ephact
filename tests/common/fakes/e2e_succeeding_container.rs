use std::collections::HashMap;

use ephact::{
    application::{
        dtos::responses::{ExecResultResponse, RunnerContextResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::{entities::FileEntry, errors::ContainerError},
};

use crate::support::container_activity::ContainerActivity;

/// Container of the scenario where every command the runner issues succeeds
/// without writing any output.
pub struct SucceedingContainer {
    activity: ContainerActivity,
}

impl SucceedingContainer {
    pub fn recording(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerPort for SucceedingContainer {
    fn exec(
        &self,
        cmd: &[String],
        _workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        self.activity.record_command(cmd, env);
        Ok(ExecResultResponse::new(0, String::new(), String::new()))
    }

    fn copy_to(&self, container_path: &str, _entries: &[FileEntry]) -> Result<(), ContainerError> {
        self.activity.record_copy(container_path);
        Ok(())
    }

    fn copy_from(&self, _container_path: &str) -> Result<Vec<FileEntry>, ContainerError> {
        Ok(Vec::new())
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
