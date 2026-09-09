use crate::application::ports::outbound::container_port::ContainerPort;

/// Request DTO for the
/// [`ResolveNodeBinaryPort`](crate::application::ports::inbound::resolve_node_binary_port::ResolveNodeBinaryPort)
/// inbound port.
pub struct ResolveNodeBinaryRequest<'a> {
    /// Container the JavaScript action will run in.
    pub container: &'a dyn ContainerPort,
}

impl<'a> ResolveNodeBinaryRequest<'a> {
    /// Creates a new request.
    pub fn new(container: &'a dyn ContainerPort) -> Self {
        Self { container }
    }

    /// Container the JavaScript action will run in.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }
}
