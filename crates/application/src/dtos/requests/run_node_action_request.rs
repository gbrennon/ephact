use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Inputs for running an action entry point with input and environment entries.
pub struct RunNodeActionRequest {
    action_dir: PathBuf,
    entry_point: String,
    inputs: HashMap<String, String>,
    env: HashMap<String, String>,
}

impl RunNodeActionRequest {
    /// Creates inputs from an action directory, entry point, input entries, and environment entries.
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

    /// Returns the action directory.
    pub fn action_dir(&self) -> &Path {
        &self.action_dir
    }

    /// Returns the entry point.
    pub fn entry_point(&self) -> &str {
        &self.entry_point
    }

    /// Returns the input entries.
    pub fn inputs(&self) -> &HashMap<String, String> {
        &self.inputs
    }

    /// Returns the environment entries.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
