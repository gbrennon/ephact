pub struct StepSummaryDetails {
    exit_code: Option<i64>,
    continue_on_error: bool,
    duration: std::time::Duration,
    stdout: String,
    stderr: String,
}

impl StepSummaryDetails {
    pub fn new(
        exit_code: Option<i64>,
        continue_on_error: bool,
        duration: std::time::Duration,
        stdout: impl Into<String>,
        stderr: impl Into<String>,
    ) -> Self {
        Self {
            exit_code,
            continue_on_error,
            duration,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }

    pub(crate) fn into_parts(self) -> (Option<i64>, bool, std::time::Duration, String, String) {
        (
            self.exit_code,
            self.continue_on_error,
            self.duration,
            self.stdout,
            self.stderr,
        )
    }
}
