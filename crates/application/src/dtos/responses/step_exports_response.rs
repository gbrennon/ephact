use std::collections::HashMap;

/// What a step exported to the steps that follow it.
#[derive(Debug, Clone, Default)]
pub struct StepExportsResponse {
    path_additions: Vec<String>,
    env: HashMap<String, String>,
    outputs: HashMap<String, String>,
}

impl StepExportsResponse {
    pub fn new(path_additions: Vec<String>, env: HashMap<String, String>) -> Self {
        Self {
            path_additions,
            env,
            outputs: HashMap::new(),
        }
    }

    pub fn with_outputs(self, outputs: HashMap<String, String>) -> Self {
        Self { outputs, ..self }
    }

    pub fn path_additions(&self) -> &[String] {
        &self.path_additions
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn outputs(&self) -> &HashMap<String, String> {
        &self.outputs
    }

    pub fn into_parts(
        self,
    ) -> (
        Vec<String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ) {
        (self.path_additions, self.env, self.outputs)
    }
}
