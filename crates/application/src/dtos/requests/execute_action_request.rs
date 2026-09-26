use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use super::execute_action_request_input::ExecuteActionRequestInput;

#[derive(Clone)]
pub struct ExecuteActionRequest {
    action_ref: String,
    step: String,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: Vec<(String, String)>,
}
pub type ExecuteActionRequestParts = (
    String,
    String,
    PathBuf,
    HashMap<String, String>,
    Vec<(String, String)>,
);

impl ExecuteActionRequest {
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

    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    pub fn step(&self) -> &str {
        &self.step
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }

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
