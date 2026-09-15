use crate::application::dtos::responses::StepSummaryResponseInput;

/// Summary of a step within a job run.
#[derive(Debug, Clone, PartialEq)]
pub struct StepSummaryResponse {
    name: String,
    step_type: crate::domain::value_objects::StepType,
    exit_code: Option<i64>,
    continue_on_error: bool,
    duration: std::time::Duration,
    stdout: String,
    stderr: String,
}

impl StepSummaryResponse {
    pub fn new(input: StepSummaryResponseInput) -> Self {
        let (name, step_type, details) = input.into_parts();
        let (exit_code, continue_on_error, duration, stdout, stderr) = details.into_parts();
        Self {
            name,
            step_type,
            exit_code,
            continue_on_error,
            duration,
            stdout,
            stderr,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn step_type(&self) -> &crate::domain::value_objects::StepType {
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
        crate::domain::value_objects::StepType,
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
