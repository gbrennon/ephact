use super::output_stream::OutputStream;

/// Payload for [`DomainEvent::StepOutput`], carrying one chunk of a step's
/// output as it is produced.
///
/// [`DomainEvent::StepOutput`]: super::domain_event::DomainEvent::StepOutput
#[derive(Debug, Clone)]
pub struct StepOutputPayload {
    /// Name of the step that produced the output.
    pub step_name: String,
    /// Stream the output was written to.
    pub stream: OutputStream,
    /// Raw text produced by the step, not necessarily line-aligned.
    pub text: String,
}
