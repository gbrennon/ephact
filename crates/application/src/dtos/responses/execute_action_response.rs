/// Response DTO for the
/// [`ExecuteActionPort`](crate::ports::inbound::execute_action_port::ExecuteActionPort)
/// inbound port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteActionResponse {
    exit_code: i64,
    stdout: String,
    stderr: String,
}

impl ExecuteActionResponse {
    pub fn new(exit_code: i64, stdout: impl Into<String>, stderr: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }

    /// Creates a builder for constructing an action response.
    pub fn builder() -> ExecuteActionResponseBuilder {
        ExecuteActionResponseBuilder::default()
    }

    /// Creates a successful response containing a user-facing note.
    pub fn note(message: impl Into<String>) -> Self {
        Self::builder().stdout(message).build()
    }

    pub fn exit_code(&self) -> i64 {
        self.exit_code
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn into_parts(self) -> (i64, String, String) {
        (self.exit_code, self.stdout, self.stderr)
    }
}

/// Immutable builder for [`ExecuteActionResponse`].
#[derive(Default)]
pub struct ExecuteActionResponseBuilder {
    exit_code: i64,
    stdout: String,
    stderr: String,
}

impl ExecuteActionResponseBuilder {
    /// Sets the process exit code.
    pub fn exit_code(mut self, exit_code: i64) -> Self {
        self.exit_code = exit_code;
        self
    }

    /// Sets standard output.
    pub fn stdout(mut self, stdout: impl Into<String>) -> Self {
        self.stdout = stdout.into();
        self
    }

    /// Sets standard error.
    pub fn stderr(mut self, stderr: impl Into<String>) -> Self {
        self.stderr = stderr.into();
        self
    }

    /// Builds the response from the configured fields.
    pub fn build(self) -> ExecuteActionResponse {
        ExecuteActionResponse::new(self.exit_code, self.stdout, self.stderr)
    }
}

#[cfg(test)]
#[path = "execute_action_response_tests.rs"]
mod tests;
