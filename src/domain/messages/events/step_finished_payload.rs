use super::StepFinishedDetails;

/// Payload for [`DomainEvent::StepFinished`].
///
/// [`DomainEvent::StepFinished`]: super::domain_event::DomainEvent::StepFinished
#[derive(Debug, Clone)]
pub struct StepFinishedPayload {
    run_id: String,
    /// Name of the workflow the step belongs to.
    workflow_name: String,
    /// Identifier of the job the step belongs to.
    job_id: String,
    /// Name of the step that finished.
    step_name: String,
    success: bool,
    exit_code: Option<i64>,
    stdout: String,
    stderr: String,
}

impl StepFinishedPayload {
    pub fn new(run_id: String, details: StepFinishedDetails) -> Self {
        Self {
            run_id,
            workflow_name: details.workflow_name().to_string(),
            job_id: details.job_id().to_string(),
            step_name: details.step_name().to_string(),
            success: details.success(),
            exit_code: details.exit_code(),
            stdout: details.stdout().to_string(),
            stderr: details.stderr().to_string(),
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn workflow_name(&self) -> &str {
        &self.workflow_name
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn step_name(&self) -> &str {
        &self.step_name
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn exit_code(&self) -> Option<i64> {
        self.exit_code
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = StepFinishedPayload::new(
            "run-1".into(),
            StepFinishedDetails::new(
                "workflow".into(),
                "job".into(),
                "step".into(),
                true,
                Some(0),
            )
            .with_stdout("stdout".into())
            .with_stderr("stderr".into()),
        );

        assert_eq!(payload.run_id(), "run-1");
        assert_eq!(payload.workflow_name(), "workflow");
        assert_eq!(payload.job_id(), "job");
        assert_eq!(payload.step_name(), "step");
        assert!(payload.success());
        assert_eq!(payload.exit_code(), Some(0));
        assert_eq!(payload.stdout(), "stdout");
        assert_eq!(payload.stderr(), "stderr");
    }
}
