use crate::application::dtos::requests::ReadStepPathExportsRequest;
use crate::application::ports::outbound::container_port::ContainerPort;

/// Inbound port for reading the `PATH` additions a step exported.
pub trait ReadStepPathExportsPort: Send + Sync {
    /// Returns the directories the step exported, or none when it exported nothing.
    fn execute(
        &self,
        request: ReadStepPathExportsRequest,
        container: &dyn ContainerPort,
    ) -> Vec<String>;
}
