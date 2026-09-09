use crate::application::ports::outbound::container_port::ContainerPort;

/// Request DTO for the
/// [`ReadStepEnvExportsPort`](crate::application::ports::inbound::read_step_env_exports_port::ReadStepEnvExportsPort)
/// inbound port.
pub struct ReadStepEnvExportsRequest<'a> {
    /// Container the step just ran in.
    container: &'a dyn ContainerPort,
}

impl<'a> ReadStepEnvExportsRequest<'a> {
    /// Creates a new request.
    pub fn new(container: &'a dyn ContainerPort) -> Self {
        Self { container }
    }

    /// Container the step just ran in.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }
}
