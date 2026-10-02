use crate::{
    domain::messages::commands::ExecuteJobCommand, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

pub trait JobCommandPublisherPort: Send + Sync {
    fn publish(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError>;
}
