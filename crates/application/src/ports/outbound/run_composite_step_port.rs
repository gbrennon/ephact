use std::sync::Arc;

use crate::{
    domain::errors::StepError,
    dtos::{requests::RunCompositeStepRequest, responses::ExecResultResponse},
    ports::outbound::ContainerPort,
};

pub trait RunCompositeStepPort: Send + Sync {
    fn run(
        &self,
        request: RunCompositeStepRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecResultResponse, StepError>;
}
