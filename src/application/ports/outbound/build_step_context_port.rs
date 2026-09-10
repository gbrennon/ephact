use crate::{application::dtos::BuildStepContextRequest, domain::value_objects::EvaluationContext};

/// Inbound port for building the context one step's expressions resolve against.
pub trait BuildStepContextPort: Send + Sync {
    /// Returns the run context with its `env` mirroring the step's environment.
    fn execute(&self, request: BuildStepContextRequest<'_>) -> EvaluationContext;
}
