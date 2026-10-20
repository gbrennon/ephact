use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use ephact::domain::{entities::Step, value_objects::EvaluationContext};

#[derive(Clone, Debug)]
pub struct DispatchedStepSnapshot {
    step: Step,
    env: HashMap<String, String>,
    context: EvaluationContext,
    repo_path: PathBuf,
}

impl DispatchedStepSnapshot {
    pub fn new(
        step: Step,
        env: HashMap<String, String>,
        context: EvaluationContext,
        repo_path: PathBuf,
    ) -> Self {
        Self {
            step,
            env,
            context,
            repo_path,
        }
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
