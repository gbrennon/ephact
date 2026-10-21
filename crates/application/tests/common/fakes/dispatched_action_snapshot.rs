use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use ephact::domain::{entities::Step, value_objects::EvaluationContext};

#[derive(Clone, Debug)]
pub struct DispatchedActionSnapshot {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
}

impl DispatchedActionSnapshot {
    pub fn new(
        action_ref: String,
        step: Step,
        repo_path: PathBuf,
        env: HashMap<String, String>,
        context: EvaluationContext,
    ) -> Self {
        Self {
            action_ref,
            step,
            repo_path,
            env,
            context,
        }
    }

    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }
}
