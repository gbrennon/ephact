use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::domain::value_objects::EvaluationContext;

/// Inputs for executing a step with repository, context, and environment values.
pub struct ExecuteStepRequest {
    step: String,
    context: EvaluationContext,
    repo_path: PathBuf,
    env: HashMap<String, String>,
}

impl ExecuteStepRequest {
    /// Creates step execution inputs.
    pub fn new(
        step: String,
        context: EvaluationContext,
        repo_path: PathBuf,
        env: HashMap<String, String>,
    ) -> Self {
        Self {
            step,
            context,
            repo_path,
            env,
        }
    }

    /// Returns the step name.
    pub fn step(&self) -> &str {
        &self.step
    }

    /// Returns the evaluation context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the environment entries.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
