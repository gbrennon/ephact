use crate::domain::value_objects::EvaluationContext;

/// Response DTO for the
/// [`BuildRunContextPort`](crate::application::ports::inbound::build_run_context_port::BuildRunContextPort)
/// outbound port.
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
