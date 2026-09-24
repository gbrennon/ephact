<<<<<<< Updated upstream
use std::collections::HashMap;

use bollard::errors::Error;
use futures_util::StreamExt;

use crate::{
    application::dtos::responses::ExecResultResponse,
    domain::{errors::ContainerError, messages::events::OutputStream},
    infrastructure::containers::bollard_wrapper::{
        Client,
        types::{CreateExecOptions, LogOutput},
    },
};

/// Builds the exec options that attach both output streams to a command.
pub(super) fn exec_options(
    cmd: &[String],
    workdir: Option<&str>,
    env: &HashMap<String, String>,
) -> CreateExecOptions<String> {
    CreateExecOptions {
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(cmd.to_vec()),
        working_dir: workdir.map(str::to_string),
        env: environment_entries(env),
        ..Default::default()
    }
}

fn environment_entries(env: &HashMap<String, String>) -> Option<Vec<String>> {
    if env.is_empty() {
        None
    } else {
        Some(
            env.iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect(),
        )
    }
}

/// Drains an attached exec output stream, relaying every stdout/stderr chunk
/// to `on_output` as it arrives and accumulating them per stream.
pub(super) async fn consume_exec_output<S>(
    output: S,
    on_output: &mut dyn FnMut(OutputStream, &str),
    container_id: &str,
) -> Result<(String, String), ContainerError>
where
    S: Unpin + StreamExt<Item = Result<LogOutput, Error>>,
{
    let mut accumulated = AccumulatedOutput::default();
    let mut output = output;
    while let Some(chunk) = output.next().await {
        relay_chunk(chunk, on_output, &mut accumulated, container_id)?;
    }
    Ok((accumulated.stdout, accumulated.stderr))
}

#[derive(Default)]
struct AccumulatedOutput {
    stdout: String,
    stderr: String,
}

fn relay_chunk(
    chunk: Result<LogOutput, Error>,
    on_output: &mut dyn FnMut(OutputStream, &str),
    accumulated: &mut AccumulatedOutput,
    container_id: &str,
) -> Result<(), ContainerError> {
    match chunk {
        Err(error) => Err(execution_failed(container_id, error)),
        Ok(LogOutput::StdOut { message }) => {
            relay(
                on_output,
                OutputStream::StandardOutput,
                &message,
                &mut accumulated.stdout,
            );
            Ok(())
        }
        Ok(LogOutput::StdErr { message }) => {
            relay(
                on_output,
                OutputStream::StandardError,
                &message,
                &mut accumulated.stderr,
            );
            Ok(())
        }
        Ok(_) => Ok(()),
    }
}

fn relay(
    on_output: &mut dyn FnMut(OutputStream, &str),
    stream: OutputStream,
    message: &bytes::Bytes,
    accumulated: &mut String,
) {
    let text = String::from_utf8_lossy(message).to_string();
    on_output(stream, &text);
    accumulated.push_str(&text);
}

/// Reads the exit code of a finished exec, defaulting to success when the
/// runtime does not report one.
pub(super) async fn exec_exit_code(client: &Client, exec_id: &str) -> i64 {
    client
        .inspect_exec(exec_id)
        .await
        .map(|inspect| inspect.exit_code.unwrap_or(0))
        .unwrap_or(0)
}

/// The result reported for a detached exec, which carries no output.
pub(super) fn detached_result() -> ExecResultResponse {
    ExecResultResponse::new(0, String::new(), String::new())
}

/// Creates the exec, streams its output, then reports the accumulated result
/// with the exit code the runtime assigned to the finished exec.
pub(super) async fn run_streaming_exec(
    client: &Client,
    container_id: &str,
    options: CreateExecOptions<String>,
    on_output: &mut dyn FnMut(OutputStream, &str),
) -> Result<ExecResultResponse, ContainerError> {
    let exec = client
        .create_exec(container_id, options)
        .await
        .map_err(|error| execution_failed(container_id, error))?;

    let started = client
        .start_exec(&exec.id, None::<bollard::exec::StartExecOptions>)
        .await
        .map_err(|error| execution_failed(container_id, error))?;

    match started {
        bollard::exec::StartExecResults::Attached { output, .. } => {
            let (stdout, stderr) = consume_exec_output(output, on_output, container_id).await?;
            Ok(ExecResultResponse::new(
                exec_exit_code(client, &exec.id).await,
                stdout,
                stderr,
            ))
        }
        bollard::exec::StartExecResults::Detached => Ok(detached_result()),
    }
}

/// Merges the container's declared environment into a runner context.
pub(super) fn runner_context_with_container_env(
    base: &crate::application::dtos::responses::RunnerContextResponse,
    container_env: Vec<String>,
) -> crate::application::dtos::responses::RunnerContextResponse {
    base.clone()
        .with_env_extension(parse_env_entries(container_env))
}

fn parse_env_entries(entries: Vec<String>) -> HashMap<String, String> {
    entries
        .iter()
        .filter_map(|kv| {
            let mut parts = kv.splitn(2, '=');
            Some((
                parts.next()?.to_string(),
                parts.next().unwrap_or("").to_string(),
            ))
        })
        .collect()
}

/// Maps a bollard failure onto the container execution error.
pub(super) fn execution_failed(container_id: &str, error: Error) -> ContainerError {
    ContainerError::ExecutionFailed(container_id.to_string(), error.to_string())
}
=======
use bollard::errors::Error;

use super::exec_output_relay::ExecOutputRelay;
use crate::{
    application::dtos::{requests::ExecOptions, responses::ExecResultResponse},
    domain::{errors::ContainerError, messages::events::OutputStream},
    infrastructure::containers::bollard_wrapper::{Client, types::CreateExecOptions},
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
>>>>>>> Stashed changes
