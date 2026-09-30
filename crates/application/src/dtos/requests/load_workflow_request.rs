pub struct LoadWorkflowRequest {
    workflow_content: String,
    file_name: String,
}

impl LoadWorkflowRequest {
    pub fn new(workflow_content: String, file_name: String) -> Self {
        Self {
            workflow_content,
            file_name,
        }
    }

    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}
