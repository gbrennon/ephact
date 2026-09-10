use std::path::Path;

/// Request DTO for the
/// [`ResolveNamedWorkflowFilePort`](crate::application::ports::inbound::resolve_named_workflow_file_port::ResolveNamedWorkflowFilePort)
/// inbound port.
pub struct ResolveNamedWorkflowFileRequest<'a> {
    /// Workflow the run was asked to execute, as named on the command line.
    workflow_name: &'a str,
    /// Path to the repository the workflow is looked up in.
    repo_path: &'a Path,
}

impl<'a> ResolveNamedWorkflowFileRequest<'a> {
    /// Creates a new request.
    pub fn new(workflow_name: &'a str, repo_path: &'a Path) -> Self {
        Self {
            workflow_name,
            repo_path,
        }
    }

    /// Workflow the run was asked to execute, as named on the command line.
    pub fn workflow_name(&self) -> &'a str {
        self.workflow_name
    }

    /// Path to the repository the workflow is looked up in.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }
}
