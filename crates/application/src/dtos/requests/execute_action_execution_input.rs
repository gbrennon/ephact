use std::{collections::HashMap, path::PathBuf};

use crate::domain::value_objects::EvaluationContext;

pub struct ExecuteActionExecutionInput {
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
}

impl ExecuteActionExecutionInput {
    pub fn new(
        repo_path: impl Into<PathBuf>,
        env: HashMap<String, String>,
        context: EvaluationContext,
    ) -> Self {
        Self {
            repo_path: repo_path.into(),
            env,
            context: context.into(),
        }
    }

    pub fn into_parts(self) -> (PathBuf, HashMap<String, String>, EvaluationContext) {
        (self.repo_path, self.env, self.context)
    }
}
