#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunConfigurationValues {
    event: String,
    inputs: Vec<(String, String)>,
}

impl RunConfigurationValues {
    pub fn new(event: String, inputs: Vec<(String, String)>) -> Self {
        Self { event, inputs }
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn inputs(&self) -> &[(String, String)] {
        &self.inputs
    }
}
