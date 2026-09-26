use std::sync::Arc;

use crate::{
    domain::errors::StepError,
    dtos::{requests::RunNodeActionRequest, responses::RunNodeActionResponse},
    ports::outbound::container_port::ContainerPort,
};

/// Runs a JavaScript action and returns its process result.
pub trait NodeActionRunnerPort: Send + Sync {
    fn run(
        &self,
        request: RunNodeActionRequest,
        container: Arc<dyn ContainerPort>,
    ) -> Result<RunNodeActionResponse, StepError>;
}
