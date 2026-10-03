use crate::{
    domain::messages::commands::ExecuteJobPayload, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

pub trait JobCommandPublisherPort: Send + Sync {
    fn publish(&self, command: ExecuteJobPayload) -> Result<JobExecutionResponse, ExecuteJobError>;
}
