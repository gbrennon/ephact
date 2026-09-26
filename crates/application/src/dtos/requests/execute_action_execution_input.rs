use std::{collections::HashMap, path::PathBuf};

pub struct ExecuteActionExecutionInput {
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: Vec<(String, String)>,
}

impl ExecuteActionExecutionInput {
    pub fn new(
        repo_path: impl Into<PathBuf>,
        env: HashMap<String, String>,
        context: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            repo_path: repo_path.into(),
            env,
            context: context.into(),
        }
    }

    pub fn into_parts(self) -> (PathBuf, HashMap<String, String>, Vec<(String, String)>) {
        (self.repo_path, self.env, self.context)
    }
}
