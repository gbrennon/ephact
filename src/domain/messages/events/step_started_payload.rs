/// Payload for [`DomainEvent::StepStarted`].
///
/// [`DomainEvent::StepStarted`]: super::domain_event::DomainEvent::StepStarted
#[derive(Debug, Clone)]
pub struct StepStartedPayload {
    /// Name of the workflow the step belongs to.
    workflow_name: String,
    /// Identifier of the job the step belongs to.
    job_id: String,
    /// Name declared by the step being run.
    step_name: String,
}

impl StepStartedPayload {
    pub fn new(workflow_name: String, job_id: String, step_name: String) -> Self {
        Self {
            workflow_name,
            job_id,
            step_name,
        }
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = StepStartedPayload::new("workflow".into(), "job".into(), "step".into());

        assert_eq!(payload.workflow_name(), "workflow");
        assert_eq!(payload.job_id(), "job");
        assert_eq!(payload.step_name(), "step");
    }
}
