/// Response DTO for the
/// [`RunNodeActionPort`](crate::application::ports::inbound::run_node_action_port::RunNodeActionPort)
/// outbound port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunNodeActionResponse {
    /// Process exit code.
    pub exit_code: i64,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
}

impl RunNodeActionResponse {
    /// Creates a new response.
    pub fn new(exit_code: i64, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
        }
    }

    /// Process exit code.
    pub fn exit_code(&self) -> i64 {
        self.exit_code
    }

    /// Captured stdout.
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Consumes the response and returns the stdout.
    pub fn into_stdout(self) -> String {
        self.stdout
    }

    /// Captured stderr.
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Consumes the response and returns the stderr.
    pub fn into_stderr(self) -> String {
        self.stderr
    }
}
