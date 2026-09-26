use crate::{
    dtos::{requests::ReadStepExportsRequest, responses::StepExportsResponse},
    ports::outbound::container_port::ContainerPort,
};

/// Reads the `PATH` and environment exports produced by a step.
pub trait StepExportsReaderPort: Send + Sync {
    /// Reads the step's `PATH` and environment exports.
    fn read(
        &self,
        request: ReadStepExportsRequest,
        container: &dyn ContainerPort,
    ) -> StepExportsResponse;
}
