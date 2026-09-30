#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowSourceFileResponse {
    content: String,
    file_name: String,
}

impl WorkflowSourceFileResponse {
    pub fn new(content: String, file_name: String) -> Self {
        Self { content, file_name }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}
