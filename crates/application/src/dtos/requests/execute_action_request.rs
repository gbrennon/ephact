use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use super::execute_action_request_input::ExecuteActionRequestInput;
use crate::domain::value_objects::EvaluationContext;

/// Request for executing an action step with repository and evaluation data.
#[derive(Clone)]
pub struct ExecuteActionRequest {
    action_ref: String,
    step: String,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
}

/// Owned values carried by an action execution request.
pub type ExecuteActionRequestParts = (
    String,
    String,
    PathBuf,
    HashMap<String, String>,
    EvaluationContext,
);

impl ExecuteActionRequest {
    /// Creates a request from its input value.
    pub fn new(input: ExecuteActionRequestInput) -> Self {
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

    /// Consumes the request and returns its parts.
    pub fn into_parts(self) -> ExecuteActionRequestParts {
        (
            self.action_ref,
            self.step,
            self.repo_path,
            self.env,
            self.context,
        )
    }
}

impl fmt::Debug for ExecuteActionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExecuteActionRequest")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}
