use crate::{
    domain::messages::commands::ExecuteJobCommand, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

/// Publishes a job command to the bound infrastructure command transport.
pub trait JobCommandPublisherPort: Send + Sync {
    fn publish(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError>;
}
