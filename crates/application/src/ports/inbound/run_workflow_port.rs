use std::{future::Future, pin::Pin};

use crate::{
    dtos::{requests::RunWorkflowRequest, responses::RunSummaryResponse},
    errors::ApplicationError,
};

pub trait RunWorkflowPort: Send + Sync {
    fn execute(
        &self,
        request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, ApplicationError>> + Send + '_>>;
}
