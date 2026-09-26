use crate::{domain::value_objects::EvaluationContext, dtos::requests::BuildStepContextRequest};

/// Builds the evaluation context for a step.
pub trait StepContextBuilderPort: Send + Sync {
    /// Returns the run context with its `env` mirroring the step's environment.
    fn build(&self, request: BuildStepContextRequest) -> EvaluationContext;
}
