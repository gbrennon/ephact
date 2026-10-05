/// Inputs for loading workflow content with its source filename.
pub struct LoadWorkflowRequest {
    workflow_content: String,
    file_name: String,
}

impl LoadWorkflowRequest {
    /// Creates inputs from workflow content and a source filename.
    pub fn new(workflow_content: String, file_name: String) -> Self {
        Self {
            workflow_content,
            file_name,
        }
    }

    /// Returns the workflow content.
    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }

    /// Returns the source filename.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}
