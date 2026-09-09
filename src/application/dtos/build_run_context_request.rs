use crate::domain::{ActRunConfig, Repository};

/// Request DTO for the
/// [`BuildRunContextPort`](crate::application::ports::inbound::build_run_context_port::BuildRunContextPort)
/// outbound port.
pub struct BuildRunContextRequest<'a> {
    /// Configuration the run was started with.
    pub config: &'a ActRunConfig,
    /// Repository the run executes against.
    pub repository: &'a Repository,
}

impl<'a> BuildRunContextRequest<'a> {
    /// Creates a new request.
    pub fn new(config: &'a ActRunConfig, repository: &'a Repository) -> Self {
        Self { config, repository }
    }

    /// Configuration the run was started with.
    pub fn config(&self) -> &'a ActRunConfig {
        self.config
    }

    /// Repository the run executes against.
    pub fn repository(&self) -> &'a Repository {
        self.repository
    }
}
