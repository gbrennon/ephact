/// Response containing workflow source content and its source file name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowSourceFileResponse {
    content: String,
    file_name: String,
}

impl WorkflowSourceFileResponse {
    /// Creates a response from workflow content and its source file name.
    pub fn new(content: String, file_name: String) -> Self {
        Self { content, file_name }
    }

    /// Returns the workflow source content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the source file name.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}
