use std::path::Path;

use crate::{
    application::dtos::ExecuteActionRequest,
    domain::{entities::Step, value_objects::EvaluationContext},
};

/// Everything needed to run a single step of a composite action.
pub struct RunCompositeStepRequest<'a> {
    step: &'a Step,

    action_dir: &'a Path,

    action_request: &'a ExecuteActionRequest,

    context: &'a EvaluationContext,
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
        }
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
