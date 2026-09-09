use crate::application::ports::outbound::container_port::ContainerPort;

/// Request DTO for the
/// [`ReadStepPathExportsPort`](crate::application::ports::inbound::read_step_path_exports_port::ReadStepPathExportsPort)
/// inbound port.
pub struct ReadStepPathExportsRequest<'a> {
    /// Container the step just ran in.
    container: &'a dyn ContainerPort,
}

impl<'a> ReadStepPathExportsRequest<'a> {
    /// Creates a new request.
    pub fn new(container: &'a dyn ContainerPort) -> Self {
        Self { container }
    }

    /// Container the step just ran in.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }
}
