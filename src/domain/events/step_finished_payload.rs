/// Payload for [`DomainEvent::StepFinished`].
///
/// [`DomainEvent::StepFinished`]: super::domain_event::DomainEvent::StepFinished
#[derive(Debug, Clone)]
pub struct StepFinishedPayload {
    /// Name of the workflow the step belongs to.
    pub workflow_name: String,
    /// Identifier of the job the step belongs to.
    pub job_id: String,
    /// Name of the step that finished.
    pub step_name: String,
    pub success: bool,
    pub exit_code: Option<i64>,
    pub stdout: String,
    pub stderr: String,
}
