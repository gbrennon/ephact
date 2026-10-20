use super::step_summary_response_input::StepSummaryResponseInput;
use crate::domain::value_objects::StepType;

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

    pub fn with_skip_reason(self, reason: String) -> Self {
        Self {
            skip_reason: Some(reason),
            ..self
        }
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::StepSummaryResponse;
    use crate::{
        domain::value_objects::StepType,
        dtos::responses::{StepExportsResponse, StepSummaryDetails, StepSummaryResponseInput},
    };

    #[test]
    fn exports_builder_replaces_outputs() {
        let response = StepExportsResponse::new(
            vec!["/bin".to_owned()],
            HashMap::from([("BASE".to_owned(), "value".to_owned())]),
        )
        .with_outputs(HashMap::from([("KEY".to_owned(), "first".to_owned())]))
        .with_outputs(HashMap::from([
            ("KEY".to_owned(), "second".to_owned()),
            ("NEW".to_owned(), "value".to_owned()),
        ]));

        assert_eq!(
            response.outputs().get("KEY").map(String::as_str),
            Some("second")
        );
        assert_eq!(
            response.outputs().get("NEW").map(String::as_str),
            Some("value")
        );
        assert_eq!(
            response.env().get("BASE").map(String::as_str),
            Some("value")
        );
    }

    #[test]
    fn summary_chaining_replaces_skip_reason() {
        let response = StepSummaryResponse::new(StepSummaryResponseInput::new(
            "build".to_owned(),
            StepType::Run,
            StepSummaryDetails::new(
                Some(0),
                false,
                std::time::Duration::from_secs(1),
                "output".to_owned(),
                String::new(),
            ),
        ))
        .with_skip_reason("first".to_owned())
        .with_skip_reason("second".to_owned());

        assert_eq!(response.skip_reason(), Some("second"));
        assert!(response.is_skipped());
    }
}
