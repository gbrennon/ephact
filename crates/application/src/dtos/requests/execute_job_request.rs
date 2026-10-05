use std::path::{Path, PathBuf};

use crate::domain::value_objects::EvaluationContext;

/// Inputs for executing a job with repository, context, identity, and permission values.
pub struct ExecuteJobRequest {
    repo_path: PathBuf,
    context: EvaluationContext,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
}

impl ExecuteJobRequest {
    /// Creates job execution inputs with network access disabled.
    pub fn new(
        repo_path: impl Into<PathBuf>,
        context: EvaluationContext,
        run_id: impl Into<String>,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            repo_path: repo_path.into(),
            context,
            run_id: run_id.into(),
            allow_repo_writes,
            allow_network: false,
        }
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the evaluation context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    /// Returns the run identifier.
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
