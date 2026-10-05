use crate::{dtos::requests::ReadStepPathExportsRequest, ports::outbound::ContainerPort};

/// Reads `PATH` additions produced by a step in a container.
pub trait ReadStepPathExportsPort: Send + Sync {
    /// Returns non-empty, trimmed lines from the step's path export file.
    /// Read or command failures produce an empty list.
    fn read(
        &self,
        request: ReadStepPathExportsRequest,
        container: &dyn ContainerPort,
    ) -> Vec<String>;
}
