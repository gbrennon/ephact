/// Payload for [`DomainEvent::JobFinished`].
///
/// [`DomainEvent::JobFinished`]: super::domain_event::DomainEvent::JobFinished
#[derive(Debug, Clone)]
pub struct JobFinishedPayload {
    /// Name of the workflow the job belongs to.
    workflow_name: String,
    /// Identifier of the job that finished.
    job_id: String,
    /// Human-readable job name, when declared.
    job_name: Option<String>,
    /// Whether the job succeeded.
    success: bool,
}

impl JobFinishedPayload {
    pub fn new(workflow_name: String, job_id: String, job_name: Option<String>, success: bool) -> Self {
        Self { workflow_name, job_id, job_name, success }
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

    pub fn success(&self) -> bool {
        self.success
    }
}
