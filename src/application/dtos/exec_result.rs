/// Result of a container exec command.
#[derive(Debug, Clone)]
pub struct ExecResult {
    pub exit_code: i64,
    pub stdout: String,
    pub stderr: String,
}

impl ExecResult {
    pub fn new(exit_code: i64, stdout: impl Into<String>, stderr: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
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
