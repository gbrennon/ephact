use std::{collections::HashMap, path::Path};

use crate::{
    domain::{entities::Step, value_objects::EvaluationContext},
    dtos::requests::ExecuteActionRequest,
};

/// Everything needed to run a single step of a composite action.
pub struct RunCompositeStepRequest<'a> {
    step: &'a Step,
    action_dir: &'a Path,
    action_request: &'a ExecuteActionRequest,
    context: &'a EvaluationContext,
    environment: Option<&'a HashMap<String, String>>,
}

impl<'a> RunCompositeStepRequest<'a> {
    pub fn new(
        step: &'a Step,
        action_dir: &'a Path,
        action_request: &'a ExecuteActionRequest,
        context: &'a EvaluationContext,
    ) -> Self {
        Self {
            step,
            action_dir,
            action_request,
            context,
            environment: None,
        }
    }

    pub fn with_environment(mut self, environment: &'a HashMap<String, String>) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn environment(&self) -> &HashMap<String, String> {
        self.environment
            .unwrap_or_else(|| self.action_request.env())
    }

    pub fn step(&self) -> &'a Step {
        self.step
    }

    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }

    pub fn action_request(&self) -> &'a ExecuteActionRequest {
        self.action_request
    }

    pub fn context(&self) -> &'a EvaluationContext {
        self.context
    }
}
