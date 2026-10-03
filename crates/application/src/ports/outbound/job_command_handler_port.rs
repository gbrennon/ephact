use crate::{
    domain::messages::commands::ExecuteJobPayload, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

pub trait JobCommandHandlerPort: Send + Sync {
    fn handle(&self, command: ExecuteJobPayload) -> Result<JobExecutionResponse, ExecuteJobError>;
}
