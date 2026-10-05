use crate::domain::value_objects::EvaluationContext;

/// Response data returned by the outbound operation.
#[derive(Debug, Clone)]
pub struct BuildRunContextResponse {
    /// The evaluated run context.
    context: EvaluationContext,
}

impl BuildRunContextResponse {
    /// Creates a new response.
    pub fn new(context: EvaluationContext) -> Self {
        Self { context }
    }

    /// The evaluated run context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    /// Consumes the response and returns the evaluated run context.
    pub fn into_context(self) -> EvaluationContext {
        self.context
    }
}
