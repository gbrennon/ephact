use std::pin::Pin;

use bollard::errors::Error;
use futures_util::{Stream, StreamExt};

use crate::{
    domain::{errors::ContainerError, messages::events::OutputStream},
    infrastructure::containers::bollard_wrapper::types::LogOutput,
};

/// Relays a container exec's attached output to a callback while accumulating
/// stdout and stderr for the final execution result.
pub struct ExecOutputRelay {
    container_id: String,
}

#[derive(Default)]
struct AccumulatedOutput {
    stdout: String,
    stderr: String,
}

impl ExecOutputRelay {
    pub fn new(container_id: String) -> Self {
        Self { container_id }
    }

    /// Drains the attached output stream, relaying each chunk as it arrives and
    /// returning the accumulated `(stdout, stderr)`.
    pub async fn consume(
        &self,
        mut output: Pin<Box<dyn Stream<Item = Result<LogOutput, Error>> + Send>>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<(String, String), ContainerError> {
        let mut accumulated = AccumulatedOutput::default();
        while let Some(chunk) = output.next().await {
            self.relay_chunk(chunk, on_output, &mut accumulated)?;
        }
        Ok((accumulated.stdout, accumulated.stderr))
    }

    fn relay_chunk(
        &self,
        chunk: Result<LogOutput, Error>,
        on_output: &mut dyn FnMut(OutputStream, &str),
        accumulated: &mut AccumulatedOutput,
    ) -> Result<(), ContainerError> {
        let (stream, message) = match chunk {
            Err(error) => {
                return Err(ContainerError::ExecutionFailed(
                    self.container_id.clone(),
                    error.to_string(),
                ));
            }
            Ok(LogOutput::StdOut { message }) => (OutputStream::StandardOutput, message),
            Ok(LogOutput::StdErr { message }) => (OutputStream::StandardError, message),
            Ok(_) => return Ok(()),
        };
        let text = String::from_utf8_lossy(&message).to_string();
        on_output(stream, &text);
        match stream {
            OutputStream::StandardOutput => accumulated.stdout.push_str(&text),
            OutputStream::StandardError => accumulated.stderr.push_str(&text),
        }
        Ok(())
    }
}
