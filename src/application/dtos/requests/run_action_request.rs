use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use super::run_action_request_input::RunActionRequestInput;

#[derive(Clone)]
pub struct RunActionRequest {
    action_ref: String,
    step: String,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: Vec<(String, String)>,
}

impl RunActionRequest {
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
