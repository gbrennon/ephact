/// Payload for [`DomainEvent::StepStarted`].
///
/// [`DomainEvent::StepStarted`]: super::domain_event::DomainEvent::StepStarted
#[derive(Debug, Clone)]
pub struct StepStartedPayload {
    /// Name of the workflow the step belongs to.
    pub workflow_name: String,
    /// Identifier of the job the step belongs to.
    pub job_id: String,
    /// Name declared by the step being run.
    pub step_name: String,
}
