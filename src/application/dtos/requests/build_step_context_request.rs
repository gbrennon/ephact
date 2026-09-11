use std::collections::HashMap;

use crate::domain::value_objects::EvaluationContext;

/// Request DTO for the
/// [`BuildStepContextPort`](crate::application::ports::inbound::build_step_context_port::BuildStepContextPort)
/// inbound port.
pub struct BuildStepContextRequest<'a> {
    /// Context of the run the step belongs to.
    context: &'a EvaluationContext,
    /// Environment the step will run with.
    env: &'a HashMap<String, String>,
}

impl<'a> BuildStepContextRequest<'a> {
    /// Creates a new request.
    pub fn new(context: &'a EvaluationContext, env: &'a HashMap<String, String>) -> Self {
        Self { context, env }
    }

    /// Context of the run the step belongs to.
    pub fn context(&self) -> &'a EvaluationContext {
        self.context
    }

    /// Environment the step will run with.
    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }
}
