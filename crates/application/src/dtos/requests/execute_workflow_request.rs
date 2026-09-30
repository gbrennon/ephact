use std::path::{Path, PathBuf};

/// Primitive request for executing a parsed workflow.
pub struct ExecuteWorkflowRequest {
    workflow_content: String,
    repo_path: PathBuf,
    context: Vec<(String, String)>,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
    file_name: Option<String>,
}

impl ExecuteWorkflowRequest {
    /// Creates a workflow execution request from owned primitive values.
    pub fn new(
        workflow_content: String,
        repo_path: PathBuf,
        context: Vec<(String, String)>,
        run_id: String,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            workflow_content,
            repo_path,
            context,
            run_id,
            allow_repo_writes,
            allow_network: false,
            file_name: None,
        }
    }

    /// Returns the workflow content.
    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }

    /// Sets the source filename used when the workflow has no YAML name.
    pub fn with_file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// Sets or clears the source filename.
    pub fn with_file_name_opt(mut self, file_name: Option<&str>) -> Self {
        self.file_name = file_name.map(str::to_owned);
        self
    }

    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns named JSON-text context roots.
    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }

    /// Returns the run identity.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Returns whether repository writes are allowed.
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    pub fn allow_network(&self) -> bool {
        self.allow_network
    }
}
