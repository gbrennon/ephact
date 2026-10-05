use std::collections::HashMap;

use crate::domain::value_objects::EvaluationContext;

/// Inputs for building a step context from an evaluation context and environment entries.
pub struct BuildStepContextRequest {
    context: EvaluationContext,
    env: HashMap<String, String>,
}

impl BuildStepContextRequest {
    /// Creates inputs from an evaluation context and environment entries.
    pub fn new(context: EvaluationContext, env: HashMap<String, String>) -> Self {
        Self { context, env }
    }

    /// Returns the evaluation context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    /// Returns the environment entries.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
