use super::output_stream::OutputStream;

/// Payload for [`DomainEvent::StepOutput`], carrying one chunk of a step's
/// output as it is produced.
///
/// [`DomainEvent::StepOutput`]: super::domain_event::DomainEvent::StepOutput
#[derive(Debug, Clone)]
pub struct StepOutputPayload {
    /// Name of the step that produced the output.
    step_name: String,
    /// Stream the output was written to.
    stream: OutputStream,
    /// Raw text produced by the step, not necessarily line-aligned.
    text: String,
}

impl StepOutputPayload {
    pub fn new(step_name: String, stream: OutputStream, text: String) -> Self {
        Self { step_name, stream, text }
    }

    pub fn step_name(&self) -> &str {
        &self.step_name
    }

    pub fn stream(&self) -> OutputStream {
        self.stream
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
