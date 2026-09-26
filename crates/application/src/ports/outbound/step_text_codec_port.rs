use ephact_domain::{entities::Step, errors::StepError};

/// Converts the external text representation of a step to and from the domain model.
pub trait StepTextCodecPort: Send + Sync {
    /// Decodes a text representation into a domain step.
    fn decode(&self, text: &str) -> Result<Step, StepError>;

    /// Encodes a domain step as text.
    fn encode(&self, step: &Step) -> Result<String, StepError>;
}
