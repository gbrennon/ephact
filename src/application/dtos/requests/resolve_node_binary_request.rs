use std::sync::Arc;

use crate::application::ports::outbound::container_port::ContainerPort;

pub struct ResolveNodeBinaryRequest {
    container: Arc<dyn ContainerPort>,
}

impl ResolveNodeBinaryRequest {
    pub fn new(container: Arc<dyn ContainerPort>) -> Self {
        Self { container }
    }

    pub fn container(&self) -> &dyn ContainerPort {
        self.container.as_ref()
    }
}
