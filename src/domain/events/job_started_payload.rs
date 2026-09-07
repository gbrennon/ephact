/// Payload for [`DomainEvent::JobStarted`].
///
/// [`DomainEvent::JobStarted`]: super::domain_event::DomainEvent::JobStarted
#[derive(Debug, Clone)]
pub struct JobStartedPayload {
    /// Name of the workflow the job belongs to.
    pub workflow_name: String,
    /// Identifier of the job in the workflow definition.
    pub job_id: String,
    /// Human-readable job name, when declared.
    pub job_name: Option<String>,
}
