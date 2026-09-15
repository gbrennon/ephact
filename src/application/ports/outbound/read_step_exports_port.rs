use crate::application::dtos::requests::ReadStepExportsRequest;
use crate::application::dtos::responses::StepExportsResponse;
use crate::application::ports::outbound::container_port::ContainerPort;

/// Inbound port for reading everything a step exported to later steps.
pub trait ReadStepExportsPort: Send + Sync {
    /// Reads the step's `PATH` and environment exports.
    fn execute(
        &self,
        request: ReadStepExportsRequest,
        container: &dyn ContainerPort,
    ) -> StepExportsResponse;
}
