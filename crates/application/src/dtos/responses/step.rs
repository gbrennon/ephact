//! Response DTOs describing individual workflow steps and their execution results.

use std::collections::HashMap;

use crate::domain::value_objects::StepType;

/// What a step exported to the steps that follow it.
#[derive(Debug, Clone, Default)]
pub struct StepExportsResponse {
    path_additions: Vec<String>,
    env: HashMap<String, String>,
    outputs: HashMap<String, String>,
}

impl StepExportsResponse {
    pub fn new(path_additions: Vec<String>, env: HashMap<String, String>) -> Self {
        Self {
            path_additions,
            env,
            outputs: HashMap::new(),
        }
    }

    pub fn with_outputs(mut self, outputs: HashMap<String, String>) -> Self {
        self.outputs = outputs;
        self
    }

    pub fn path_additions(&self) -> &[String] {
        &self.path_additions
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn outputs(&self) -> &HashMap<String, String> {
        &self.outputs
    }

    pub fn into_parts(
        self,
    ) -> (
        Vec<String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ) {
        (self.path_additions, self.env, self.outputs)
    }
}

/// Details captured while executing a step.
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

/// Input values used to construct a step summary.
pub struct StepSummaryResponseInput {
    name: String,
    step_type: StepType,
    details: StepSummaryDetails,
}

impl StepSummaryResponseInput {
    pub fn new(name: impl Into<String>, step_type: StepType, details: StepSummaryDetails) -> Self {
        Self {
            name: name.into(),
            step_type,
            details,
        }
    }

    pub(crate) fn into_parts(self) -> (String, StepType, StepSummaryDetails) {
        (self.name, self.step_type, self.details)
    }
}

/// Summary of a step within a job run.
#[derive(Debug, Clone, PartialEq)]
pub struct StepSummaryResponse {
    name: String,
    step_type: StepType,
    exit_code: Option<i64>,
    continue_on_error: bool,
    duration: std::time::Duration,
    stdout: String,
    stderr: String,
    skip_reason: Option<String>,
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
            skip_reason: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn step_type(&self) -> &StepType {
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
    pub fn is_skipped(&self) -> bool {
        self.skip_reason.is_some()
    }
    pub fn skip_reason(&self) -> Option<&str> {
        self.skip_reason.as_deref()
    }

    pub fn with_skip_reason(mut self, reason: impl Into<String>) -> Self {
        self.skip_reason = Some(reason.into());
        self
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        StepType,
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

/// Summary of one executed step, and whether it fails the job it belongs to.
pub struct SummarizedStepResponse {
    summary: StepSummaryResponse,
    fails_job: bool,
}

impl SummarizedStepResponse {
    pub fn new(summary: StepSummaryResponse, fails_job: bool) -> Self {
        Self { summary, fails_job }
    }
    pub fn summary(&self) -> &StepSummaryResponse {
        &self.summary
    }
    pub fn into_summary(self) -> StepSummaryResponse {
        self.summary
    }
    pub fn fails_job(&self) -> bool {
        self.fails_job
    }
}
