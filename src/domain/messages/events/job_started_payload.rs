/// Payload for [`DomainEvent::JobStarted`].
///
/// [`DomainEvent::JobStarted`]: super::domain_event::DomainEvent::JobStarted
#[derive(Debug, Clone)]
pub struct JobStartedPayload {
    /// Name of the workflow the job belongs to.
    workflow_name: String,
    /// Identifier of the job in the workflow definition.
    job_id: String,
    /// Human-readable job name, when declared.
    job_name: Option<String>,
}

impl JobStartedPayload {
    pub fn new(workflow_name: String, job_id: String, job_name: Option<String>) -> Self {
        Self {
            workflow_name,
            job_id,
            job_name,
        }
    }

    pub fn workflow_name(&self) -> &str {
        &self.workflow_name
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn job_name(&self) -> Option<&str> {
        self.job_name.as_deref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = JobStartedPayload::new("workflow".into(), "job".into(), Some("build".into()));

        assert_eq!(payload.workflow_name(), "workflow");
        assert_eq!(payload.job_id(), "job");
        assert_eq!(payload.job_name(), Some("build"));
    }
}
