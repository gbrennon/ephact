use crate::domain::expression::EvalContext;

/// Response DTO for the
/// [`BuildRunContextPort`](crate::application::ports::inbound::build_run_context_port::BuildRunContextPort)
/// outbound port.
#[derive(Debug, Clone)]
pub struct BuildRunContextResponse {
    /// The evaluated run context.
    context: EvalContext,
}

impl BuildRunContextResponse {
    /// Creates a new response.
    pub fn new(context: EvalContext) -> Self {
        Self { context }
    }

    /// The evaluated run context.
    pub fn context(&self) -> &EvalContext {
        &self.context
    }

    /// Consumes the response and returns the evaluated run context.
    pub fn into_context(self) -> EvalContext {
        self.context
    }
}
