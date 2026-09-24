use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::RunCompositeActionRequest, responses::ExecuteActionResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::StepError,
};
/// Runs the steps that define a composite action.
pub trait CompositeActionRunnerPort: Send + Sync {
    fn run(
        &self,
        request: RunCompositeActionRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
