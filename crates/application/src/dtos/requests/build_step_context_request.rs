use std::collections::HashMap;

use crate::domain::value_objects::EvaluationContext;
pub struct BuildStepContextRequest {
    context: EvaluationContext,
    env: HashMap<String, String>,
}

impl BuildStepContextRequest {
    pub fn new(context: EvaluationContext, env: HashMap<String, String>) -> Self {
        Self { context, env }
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
