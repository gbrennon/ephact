/// Response DTO for the
/// [`ExecuteActionPort`](crate::ports::inbound::execute_action_port::ExecuteActionPort)
/// inbound port.
#[path = "execute_action_response_builder.rs"]
mod execute_action_response_builder;

pub use execute_action_response_builder::ExecuteActionResponseBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteActionResponse {
    exit_code: i64,
    stdout: String,
    stderr: String,
}

impl ExecuteActionResponse {
    /// Creates a response from an exit code, standard output, and standard error.
    pub fn new(exit_code: i64, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
        }
    }

    /// Creates a builder for constructing an action response.
    pub fn builder() -> ExecuteActionResponseBuilder {
        ExecuteActionResponseBuilder::default()
    }

    /// Creates a successful response containing a user-facing note.
    pub fn note(message: String) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_succeeds_and_carries_the_message() {
        let response = ExecuteActionResponse::note("workspace already mounted\n".to_owned());

        assert_eq!(response.exit_code(), 0);
        assert_eq!(response.stdout(), "workspace already mounted\n");
        assert!(response.stderr().is_empty());
    }

    #[test]
    fn builder_constructs_all_response_fields() {
        let response = ExecuteActionResponse::builder()
            .exit_code(2)
            .stdout("output".to_owned())
            .stderr("error".to_owned())
            .build();

        assert_eq!(response.into_parts(), (2, "output".into(), "error".into()));
    }
}
