use crate::application::ports::outbound::container_port::ContainerPort;

/// Request DTO for the
/// [`ReadStepExportsPort`](crate::application::ports::inbound::read_step_exports_port::ReadStepExportsPort)
/// inbound port.
pub struct ReadStepExportsRequest<'a> {
    /// Container the step just ran in.
    container: &'a dyn ContainerPort,
}

impl<'a> ReadStepExportsRequest<'a> {
    /// Creates a new request.
    pub fn new(container: &'a dyn ContainerPort) -> Self {
        Self { container }
    }

    /// Container the step just ran in.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }
}
