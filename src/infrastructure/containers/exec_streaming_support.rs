use std::collections::HashMap;

use bollard::errors::Error;
use futures_util::StreamExt;

use crate::{
    application::dtos::ExecResult,
    domain::errors::ContainerError,
    domain::events::OutputStream,
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
pub(super) fn detached_result() -> ExecResult {
    ExecResult {
        exit_code: 0,
        stdout: String::new(),
        stderr: String::new(),
    }
}

/// Creates the exec, streams its output, then reports the accumulated result
/// with the exit code the runtime assigned to the finished exec.
pub(super) async fn run_streaming_exec(
    client: &Client,
    container_id: &str,
    options: CreateExecOptions<String>,
    on_output: &mut dyn FnMut(OutputStream, &str),
) -> Result<ExecResult, ContainerError> {
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
            Ok(ExecResult {
                exit_code: exec_exit_code(client, &exec.id).await,
                stdout,
                stderr,
            })
        }
        bollard::exec::StartExecResults::Detached => Ok(detached_result()),
    }
}

/// Merges the container's declared environment into a runner context.
pub(super) fn runner_context_with_container_env(
    base: &crate::application::dtos::RunnerContext,
    container_env: Vec<String>,
) -> crate::application::dtos::RunnerContext {
    let mut context = base.clone();
    context.env.extend(parse_env_entries(container_env));
    context
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
