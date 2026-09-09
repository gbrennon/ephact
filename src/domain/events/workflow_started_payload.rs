/// Payload for [`DomainEvent::WorkflowStarted`].
///
/// [`DomainEvent::WorkflowStarted`]: super::domain_event::DomainEvent::WorkflowStarted
#[derive(Debug, Clone)]
pub struct WorkflowStartedPayload {
    /// Name declared by the workflow being run.
    workflow_name: String,
}

impl WorkflowStartedPayload {
    pub fn new(workflow_name: String) -> Self {
        Self { workflow_name }
    }

    pub fn workflow_name(&self) -> &str {
        &self.workflow_name
    }
}
