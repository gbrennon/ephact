pub struct LoadWorkflowRequest<'a> {
    workflow_content: &'a str,
}

impl<'a> LoadWorkflowRequest<'a> {
    pub fn new(workflow_content: &'a str) -> Self {
        Self { workflow_content }
    }

    pub fn workflow_content(&self) -> &'a str {
        self.workflow_content
    }
}
