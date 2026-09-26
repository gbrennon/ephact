use crate::{
    domain::messages::commands::ExecuteJobCommand, dtos::responses::JobExecutionResponse,
    errors::ExecuteJobError,
};

pub trait JobCommandBusPort: Send + Sync {
    fn dispatch(&self, command: ExecuteJobCommand)
    -> Result<JobExecutionResponse, ExecuteJobError>;
}
