use std::collections::HashMap;

/// What a step exported to the steps that follow it.
#[derive(Debug, Clone, Default)]
pub struct StepExports {
    pub path_additions: Vec<String>,
    pub env: HashMap<String, String>,
}

impl StepExports {
    pub fn new(path_additions: Vec<String>, env: HashMap<String, String>) -> Self {
        Self {
            path_additions,
            env,
        }
    }

    pub fn path_additions(&self) -> &[String] {
        &self.path_additions
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn into_parts(self) -> (Vec<String>, HashMap<String, String>) {
        (self.path_additions, self.env)
    }
}
