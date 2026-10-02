use std::collections::HashMap;

use crate::{
    dtos::requests::ReadStepOutputExportsRequest, ports::outbound::container_port::ContainerPort,
};

/// Reads values a step exported for use by later steps.
pub trait ReadStepOutputExportsPort: Send + Sync {
    fn read(
        &self,
        request: ReadStepOutputExportsRequest,
        container: &dyn ContainerPort,
    ) -> HashMap<String, String>;
}
