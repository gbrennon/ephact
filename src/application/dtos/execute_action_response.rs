/// Response DTO for the
/// [`ExecuteActionPort`](crate::application::ports::inbound::execute_action_port::ExecuteActionPort)
/// inbound port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteActionResponse {
    pub exit_code: i64,
    pub stdout: String,
    pub stderr: String,
}

impl ExecuteActionResponse {
    pub fn new(exit_code: i64, stdout: impl Into<String>, stderr: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }

    pub fn note(message: impl Into<String>) -> Self {
        Self {
            exit_code: 0,
            stdout: message.into(),
            stderr: String::new(),
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
}
