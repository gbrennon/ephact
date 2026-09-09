/// Summary of a step within a job run.
#[derive(Debug, Clone, PartialEq)]
pub struct StepSummary {
    pub name: String,
    pub step_type: crate::domain::workflow::StepType,
    pub exit_code: Option<i64>,
    pub continue_on_error: bool,
    pub duration: std::time::Duration,
    pub stdout: String,
    pub stderr: String,
}

impl StepSummary {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        step_type: crate::domain::workflow::StepType,
        exit_code: Option<i64>,
        continue_on_error: bool,
        duration: std::time::Duration,
        stdout: impl Into<String>,
        stderr: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            step_type,
            exit_code,
            continue_on_error,
            duration,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn step_type(&self) -> &crate::domain::workflow::StepType {
        &self.step_type
    }

    pub fn exit_code(&self) -> Option<i64> {
        self.exit_code
    }

    pub fn continue_on_error(&self) -> bool {
        self.continue_on_error
    }

    pub fn duration(&self) -> std::time::Duration {
        self.duration
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        crate::domain::workflow::StepType,
        Option<i64>,
        bool,
        std::time::Duration,
        String,
        String,
    ) {
        (
            self.name,
            self.step_type,
            self.exit_code,
            self.continue_on_error,
            self.duration,
            self.stdout,
            self.stderr,
        )
    }
}
