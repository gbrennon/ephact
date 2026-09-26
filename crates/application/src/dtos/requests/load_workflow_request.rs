pub struct LoadWorkflowRequest {
    workflow_content: String,
}

impl LoadWorkflowRequest {
    pub fn new(workflow_content: String) -> Self {
        Self { workflow_content }
    }

    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }
}
