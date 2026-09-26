use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct ExecuteStepRequest {
    step: String,
    context: Vec<(String, String)>,
    repo_path: PathBuf,
    env: HashMap<String, String>,
}

impl ExecuteStepRequest {
    pub fn new(
        step: impl Into<String>,
        context: Vec<(String, String)>,
        repo_path: impl Into<PathBuf>,
        env: HashMap<String, String>,
    ) -> Self {
        Self {
            step: step.into(),
            context,
            repo_path: repo_path.into(),
            env,
        }
    }

    pub fn step(&self) -> &str {
        &self.step
    }
    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
