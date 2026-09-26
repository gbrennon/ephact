use crate::domain::{ActRunConfig, Repository};

/// Request DTO for the
/// [`BuildRunContextPort`](crate::ports::inbound::build_run_context_port::BuildRunContextPort)
/// outbound port.
pub struct BuildRunContextRequest {
    config: ActRunConfig,
    repository: Repository,
}

impl BuildRunContextRequest {
    pub fn new(config: ActRunConfig, repository: Repository) -> Self {
        Self { config, repository }
    }

    pub fn config(&self) -> &ActRunConfig {
        &self.config
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}
