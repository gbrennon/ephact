use crate::{
    domain::messages::commands::ExecuteJobCommand, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

pub trait JobCommandHandlerPort: Send + Sync {
    fn handle(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError>;
}
