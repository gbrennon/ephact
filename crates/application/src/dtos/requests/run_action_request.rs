use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use super::run_action_request_input::RunActionRequestInput;
use crate::domain::value_objects::EvaluationContext;

/// Request for running an action step with repository and evaluation data.
#[derive(Clone)]
pub struct RunActionRequest {
    action_ref: String,
    step: String,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
}

impl RunActionRequest {
    /// Creates a request from its input value.
    pub fn new(input: RunActionRequestInput) -> Self {
        let (action_ref, step, execution) = input.into_parts();
        let (repo_path, env, context) = execution.into_parts();
        Self {
            action_ref,
            step,
            repo_path,
            env,
            context,
        }
    }

    /// Returns the action reference.
    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    /// Returns the step name.
    pub fn step(&self) -> &str {
        &self.step
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the environment entries.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    /// Returns the evaluation context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }
}

impl fmt::Debug for RunActionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RunActionRequest")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}
