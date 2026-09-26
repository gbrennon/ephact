use bollard::errors::Error;

use super::exec_output_relay::ExecOutputRelay;
use crate::{
    application::dtos::{requests::ExecOptions, responses::ExecResultResponse},
    containers::bollard_wrapper::{Client, types::CreateExecOptions},
    domain::{errors::ContainerError, messages::events::OutputStream},
};

/// Runs streaming exec commands against a specific container.
pub struct ExecStreamingSupport {
    client: Client,
    container_id: String,
}

impl ExecStreamingSupport {
    pub fn new(client: Client, container_id: String) -> Self {
        Self {
            client,
            container_id,
        }
    }

    /// Creates the exec, streams its output to `on_output`, then reports the
    /// accumulated result with the exit code assigned to the finished exec.
    pub async fn run(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        let exec = self
            .client
            .create_exec(&self.container_id, self.exec_options(&options))
            .await
            .map_err(|error| self.execution_failed(error))?;
        let started = self
            .client
            .start_exec(&exec.id, None::<bollard::exec::StartExecOptions>)
            .await
            .map_err(|error| self.execution_failed(error))?;
        match started {
            bollard::exec::StartExecResults::Attached { output, .. } => {
                let (stdout, stderr) = ExecOutputRelay::new(self.container_id.clone())
                    .consume(output, on_output)
                    .await?;
                let exit_code = self.exec_exit_code(&exec.id).await;
                Ok(ExecResultResponse::new(exit_code, stdout, stderr))
            }
            bollard::exec::StartExecResults::Detached => {
                Ok(ExecResultResponse::new(0, String::new(), String::new()))
            }
        }
    }

    fn exec_options(&self, options: &ExecOptions<'_>) -> CreateExecOptions<String> {
        let environment = (!options.env().is_empty()).then(|| {
            options
                .env()
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect()
        });
        CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(options.cmd().to_vec()),
            working_dir: options.workdir().map(str::to_string),
            env: environment,
            ..Default::default()
        }
    }

    async fn exec_exit_code(&self, exec_id: &str) -> i64 {
        self.client
            .inspect_exec(exec_id)
            .await
            .map(|inspect| inspect.exit_code.unwrap_or(0))
            .unwrap_or(0)
    }

    fn execution_failed(&self, error: Error) -> ContainerError {
        ContainerError::ExecutionFailed(self.container_id.clone(), error.to_string())
    }
}
