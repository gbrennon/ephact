/// Error raised while executing a workflow step, carrying any partial output
/// produced before the failure so it can be surfaced in the run summary.
#[derive(Debug)]
pub struct StepError {
    pub message: String,
    pub stdout: String,
    pub stderr: String,
}

impl StepError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn with_stdout(mut self, stdout: impl Into<String>) -> Self {
        self.stdout = stdout.into();
        self
    }

    pub fn with_stderr(mut self, stderr: impl Into<String>) -> Self {
        self.stderr = stderr.into();
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_initializes_message_and_empty_buffers() {
        let err = StepError::new("foo");

        assert_eq!(err.message(), "foo");
        assert!(err.stdout().is_empty(), "stdout should be emtpy");
        assert!(err.stderr().is_empty(), "stderr should be empty");
    }

    #[test]
    fn new_accepts_string() {
        let err = StepError::new(String::from("bar"));

        assert_eq!(err.message(), "bar");
    }

    #[test]
    fn with_stdout_and_with_stderr_attach_output() {
        let err = StepError::new("test")
            .with_stdout("output")
            .with_stderr("error");

        assert_eq!(err.stdout(), "output");
        assert_eq!(err.stderr(), "error");
    }

    #[test]
    fn debug_impl() {
        let err = StepError::new("debug test");
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("StepError"));
        assert!(debug_str.contains("debug test"));
    }
}
