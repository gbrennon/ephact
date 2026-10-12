/// Result of a container exec command.
#[derive(Debug, Clone)]
pub struct ExecResultResponse {
    exit_code: i64,
    stdout: String,
    stderr: String,
}

impl ExecResultResponse {
    pub fn new(exit_code: i64, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
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
