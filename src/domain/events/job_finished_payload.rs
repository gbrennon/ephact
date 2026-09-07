/// Payload for [`DomainEvent::JobFinished`].
///
/// [`DomainEvent::JobFinished`]: super::domain_event::DomainEvent::JobFinished
#[derive(Debug, Clone)]
pub struct JobFinishedPayload {
    /// Name of the workflow the job belongs to.
    pub workflow_name: String,
    /// Identifier of the job that finished.
    pub job_id: String,
    /// Human-readable job name, when declared.
    pub job_name: Option<String>,
    /// Whether the job succeeded.
    pub success: bool,
}
