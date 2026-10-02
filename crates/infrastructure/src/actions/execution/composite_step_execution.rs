use std::{collections::HashMap, path::Path};

use super::run_composite_action_service::ActionOutput;
use crate::domain::value_objects::EvaluationContext;

pub(super) struct CompositeStepExecution<'a> {
    action_dir: &'a Path,
    context: &'a mut EvaluationContext,
    output: &'a mut ActionOutput,
    environment: &'a mut HashMap<String, String>,
}

impl<'a> CompositeStepExecution<'a> {
    pub(super) fn new(
        action_dir: &'a Path,
        context: &'a mut EvaluationContext,
        output: &'a mut ActionOutput,
        environment: &'a mut HashMap<String, String>,
    ) -> Self {
        Self {
            action_dir,
            context,
            output,
            environment,
        }
    }

    pub(super) fn action_dir(&self) -> &Path {
        self.action_dir
    }

    pub(super) fn context(&self) -> &EvaluationContext {
        self.context
    }

    pub(super) fn output(&self) -> &ActionOutput {
        self.output
    }

    pub(super) fn output_mut(&mut self) -> &mut ActionOutput {
        self.output
    }

    pub(super) fn environment_and_context_mut(
        &mut self,
    ) -> (&mut HashMap<String, String>, &mut EvaluationContext) {
        (self.environment, self.context)
    }

    pub(super) fn environment(&mut self) -> &mut HashMap<String, String> {
        self.environment
    }
}
