use crate::{
    application::dtos::requests::BuildStepContextRequest, domain::value_objects::EvaluationContext,
};

/// Builds the evaluation context for a step.
pub trait StepContextBuilderPort: Send + Sync {
    /// Returns the run context with its `env` mirroring the step's environment.
    fn build(&self, request: BuildStepContextRequest) -> EvaluationContext;
}
