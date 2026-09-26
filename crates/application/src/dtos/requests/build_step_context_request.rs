use std::collections::HashMap;

pub struct BuildStepContextRequest {
    context: Vec<(String, String)>,
    env: HashMap<String, String>,
}

impl BuildStepContextRequest {
    pub fn new(context: Vec<(String, String)>, env: HashMap<String, String>) -> Self {
        Self { context, env }
    }

    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
