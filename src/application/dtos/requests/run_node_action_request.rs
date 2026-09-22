use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct RunNodeActionRequest {
    action_dir: PathBuf,
    entry_point: String,
    inputs: HashMap<String, String>,
    env: HashMap<String, String>,
}

impl RunNodeActionRequest {
    pub fn new(
        action_dir: impl Into<PathBuf>,
        entry_point: impl Into<String>,
        inputs: HashMap<String, String>,
        env: HashMap<String, String>,
    ) -> Self {
        Self {
            action_dir: action_dir.into(),
            entry_point: entry_point.into(),
            inputs,
            env,
        }
    }

    pub fn action_dir(&self) -> &Path {
        &self.action_dir
    }
    pub fn entry_point(&self) -> &str {
        &self.entry_point
    }
    pub fn inputs(&self) -> &HashMap<String, String> {
        &self.inputs
    }
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
