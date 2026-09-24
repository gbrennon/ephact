use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::RunNodeActionRequest, responses::RunNodeActionResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::StepError,
};

/// Runs a JavaScript action and returns its process result.
pub trait NodeActionRunnerPort: Send + Sync {
    fn run(
        &self,
        request: RunNodeActionRequest,
        container: Arc<dyn ContainerPort>,
    ) -> Result<RunNodeActionResponse, StepError>;
}
