use std::collections::HashMap;

use crate::application::dtos::requests::ReadStepEnvExportsRequest;
use crate::application::ports::outbound::container_port::ContainerPort;

/// Inbound port for reading the environment variables a step exported.
pub trait ReadStepEnvExportsPort: Send + Sync {
    /// Returns the variables the step exported, or none when it exported nothing.
    fn execute(
        &self,
        request: ReadStepEnvExportsRequest,
        container: &dyn ContainerPort,
    ) -> HashMap<String, String>;
}
