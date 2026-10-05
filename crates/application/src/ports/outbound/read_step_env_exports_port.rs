use std::collections::HashMap;

use crate::{dtos::requests::ReadStepEnvExportsRequest, ports::outbound::ContainerPort};

/// Reads environment exports produced by a step in a container.
pub trait ReadStepEnvExportsPort: Send + Sync {
    /// Reads and parses the step export file, returning user exports while
    /// omitting reserved `GITHUB_`, `RUNNER_`, and `NODE_OPTIONS` entries.
    /// Read or command failures produce an empty map.
    fn read(
        &self,
        request: ReadStepEnvExportsRequest,
        container: &dyn ContainerPort,
    ) -> HashMap<String, String>;
}
