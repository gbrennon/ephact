use std::sync::Arc;

use crate::application::dtos::requests::RunCompositeStepRequest;
use crate::application::dtos::responses::ExecResultResponse;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::StepError;

/// Inbound port for running one step of a composite action.
pub trait RunCompositeStepPort: Send + Sync {
    fn execute(
        &self,
        request: RunCompositeStepRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecResultResponse, StepError>;
}
