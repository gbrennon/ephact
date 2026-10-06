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
    /// Creates a response from an exit code, standard output, and standard error.
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

    /// Returns the process exit code.
    pub fn exit_code(&self) -> i64 {
        self.exit_code
    }

    /// Returns standard output.
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns standard error.
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Consumes the response and returns its exit code and output streams.
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
mod tests {
    use super::*;

    #[test]
    fn note_succeeds_and_carries_the_message() {
        let response = ExecuteActionResponse::note("workspace already mounted\n");

        assert_eq!(response.exit_code(), 0);
        assert_eq!(response.stdout(), "workspace already mounted\n");
        assert!(response.stderr().is_empty());
    }

    #[test]
    fn builder_constructs_all_response_fields() {
        let response = ExecuteActionResponse::builder()
            .exit_code(2)
            .stdout("output")
            .stderr("error")
            .build();

        assert_eq!(response.into_parts(), (2, "output".into(), "error".into()));
    }
}
