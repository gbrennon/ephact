use crate::{
    application::{dtos::responses::JobExecutionResponse, errors::ExecuteJobError},
    domain::messages::commands::ExecuteJobCommand,
};

pub trait JobCommandBusPort: Send + Sync {
    fn dispatch(&self, command: ExecuteJobCommand)
    -> Result<JobExecutionResponse, ExecuteJobError>;
}
