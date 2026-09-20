use std::future::Future;
use std::pin::Pin;

use crate::application::dtos::requests::RunWorkflowRequest;
use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::errors::RunWorkflowError;

pub trait RunWorkflowPort: Send + Sync {
    fn execute(
        &self,
        request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, RunWorkflowError>> + Send + '_>>;
}
