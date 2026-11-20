use std::{future::Future, pin::Pin};

use crate::application::{
    dtos::{requests::RunWorkflowRequest, responses::RunSummaryResponse},
    errors::ApplicationError,
    ports::inbound::RunWorkflowPort,
};

pub struct UnusedRunWorkflowPort;

impl RunWorkflowPort for UnusedRunWorkflowPort {
    fn execute(
        &self,
        _request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, ApplicationError>> + Send + '_>>
    {
        Box::pin(async { Err(ApplicationError::Workflow("unused".to_string())) })
    }
}
