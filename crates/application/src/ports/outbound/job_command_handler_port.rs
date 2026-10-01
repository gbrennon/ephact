use crate::{
    domain::messages::commands::ExecuteJobCommand, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

/// Handles a job command routed by the infrastructure command transport.
pub trait JobCommandHandlerPort: Send + Sync {
    fn handle(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError>;
}
