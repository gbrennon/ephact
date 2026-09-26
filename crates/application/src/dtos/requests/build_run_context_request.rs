use crate::domain::{Repository, WorkflowRunConfig};

/// Request DTO for the
/// [`BuildRunContextPort`](crate::ports::inbound::build_run_context_port::BuildRunContextPort)
/// outbound port.
pub struct BuildRunContextRequest {
    config: WorkflowRunConfig,
    repository: Repository,
}

impl BuildRunContextRequest {
    pub fn new(config: WorkflowRunConfig, repository: Repository) -> Self {
        Self { config, repository }
    }

    pub fn config(&self) -> &WorkflowRunConfig {
        &self.config
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}
