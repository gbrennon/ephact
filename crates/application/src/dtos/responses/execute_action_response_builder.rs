use super::execute_action_response::ExecuteActionResponse;

/// Immutable builder for [`ExecuteActionResponse`].
#[derive(Default)]
pub struct ExecuteActionResponseBuilder {
    exit_code: i64,
    stdout: String,
    stderr: String,
}

impl ExecuteActionResponseBuilder {
    /// Sets the process exit code.
    pub fn exit_code(self, exit_code: i64) -> Self {
        Self { exit_code, ..self }
    }

    /// Sets standard output.
    pub fn stdout(self, stdout: String) -> Self {
        Self { stdout, ..self }
    }

    /// Sets standard error.
    pub fn stderr(self, stderr: String) -> Self {
        Self { stderr, ..self }
    }

    /// Builds the response from the configured fields.
    pub fn build(self) -> ExecuteActionResponse {
        ExecuteActionResponse::new(self.exit_code, self.stdout, self.stderr)
    }
}
