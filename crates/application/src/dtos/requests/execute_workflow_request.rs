use std::path::{Path, PathBuf};

use crate::domain::value_objects::EvaluationContext;

/// Inputs for executing workflow content with repository, context, identity, and permission values.
pub struct ExecuteWorkflowRequest {
    workflow_content: String,
    repo_path: PathBuf,
    context: EvaluationContext,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
    file_name: Option<String>,
    selected_job: Option<String>,
}

impl ExecuteWorkflowRequest {
    /// Creates a workflow execution request from owned values.
    pub fn new(
        workflow_content: String,
        repo_path: PathBuf,
        context: EvaluationContext,
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
            selected_job: None,
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

    /// Returns the optional source filename.
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// Sets the selected job used to limit workflow execution.
    pub fn with_selected_job(mut self, selected_job: impl Into<String>) -> Self {
        self.selected_job = Some(selected_job.into());
        self
    }

    /// Sets or clears the selected job used to limit workflow execution.
    pub fn with_selected_job_opt(mut self, selected_job: Option<&str>) -> Self {
        self.selected_job = selected_job.map(str::to_owned);
        self
    }

    /// Returns the selected job used to limit workflow execution.
    pub fn selected_job(&self) -> Option<&str> {
        self.selected_job.as_deref()
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the workflow evaluation context.
    pub fn context(&self) -> &EvaluationContext {
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

    /// Sets whether network access is allowed.
    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    /// Returns whether network access is allowed.
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }
}
