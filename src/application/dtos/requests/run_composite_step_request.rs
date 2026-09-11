use std::path::Path;

use crate::application::dtos::requests::ExecuteActionRequest;
use crate::domain::entities::Step;
use crate::domain::value_objects::EvaluationContext;

/// Everything needed to run a single step of a composite action.
pub struct RunCompositeStepRequest<'a> {
    step: &'a Step,
    action_dir: &'a Path,
    action_request: &'a ExecuteActionRequest<'a>,
    context: &'a EvaluationContext,
}

impl<'a> RunCompositeStepRequest<'a> {
    pub fn new(
        step: &'a Step,
        action_dir: &'a Path,
        action_request: &'a ExecuteActionRequest<'a>,
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

    pub fn action_request(&self) -> &'a ExecuteActionRequest<'a> {
        self.action_request
    }

    pub fn context(&self) -> &'a EvaluationContext {
        self.context
    }
}
