use std::path::{Path, PathBuf};

use crate::domain::value_objects::EvaluationContext;

/// Inputs for executing a job with repository, context, identity, and permission values.
pub struct ExecuteJobRequest {
    repo_path: PathBuf,
    context: EvaluationContext,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
    event_payload: Option<String>,
}

impl ExecuteJobRequest {
    /// Creates job execution inputs with network access disabled.
    pub fn new(
        repo_path: PathBuf,
        context: EvaluationContext,
        run_id: String,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            repo_path,
            context,
            run_id,
            allow_repo_writes,
            allow_network: false,
            event_payload: None,
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

    /// Sets or clears the raw event payload carried into the job container.
    pub fn with_event_payload_opt(self, event_payload: Option<&str>) -> Self {
        Self {
            event_payload: event_payload.map(str::to_owned),
            ..self
        }
    }

    /// Returns the raw event payload carried into the job container.
    pub fn event_payload(&self) -> Option<&str> {
        self.event_payload.as_deref()
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
    pub fn with_allow_network(self, allow_network: bool) -> Self {
        Self {
            allow_network,
            ..self
        }
    }

    /// Returns whether network access is allowed.
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }
}
