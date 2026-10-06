use std::{collections::HashMap, path::Path};

use crate::{
    domain::{entities::Step, value_objects::EvaluationContext},
    dtos::requests::ExecuteActionRequest,
};

/// Inputs for running one step of a composite action.
pub struct RunCompositeStepRequest<'a> {
    step: &'a Step,
    action_dir: &'a Path,
    action_request: &'a ExecuteActionRequest,
    context: &'a EvaluationContext,
    environment: Option<&'a HashMap<String, String>>,
}

impl<'a> RunCompositeStepRequest<'a> {
    /// Creates inputs from a step, action directory, action request, and evaluation context.
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

    /// Sets an environment override.
    pub fn with_environment(mut self, environment: &'a HashMap<String, String>) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Returns the configured environment or the action request environment.
    pub fn environment(&self) -> &HashMap<String, String> {
        self.environment
            .unwrap_or_else(|| self.action_request.env())
    }

    /// Returns the step.
    pub fn step(&self) -> &'a Step {
        self.step
    }

    /// Returns the action directory.
    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }

    /// Returns the action request.
    pub fn action_request(&self) -> &'a ExecuteActionRequest {
        self.action_request
    }

    /// Returns the evaluation context.
    pub fn context(&self) -> &'a EvaluationContext {
        self.context
    }
}
