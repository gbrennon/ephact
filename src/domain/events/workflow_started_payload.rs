/// Payload for [`DomainEvent::WorkflowStarted`].
///
/// [`DomainEvent::WorkflowStarted`]: super::domain_event::DomainEvent::WorkflowStarted
#[derive(Debug, Clone)]
pub struct WorkflowStartedPayload {
    /// Name declared by the workflow being run.
    pub workflow_name: String,
}
