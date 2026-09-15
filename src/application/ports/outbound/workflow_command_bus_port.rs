use crate::{
    application::{dtos::responses::WorkflowExecutionResponse, errors::ExecuteWorkflowError},
    domain::messages::commands::ExecuteWorkflowCommand,
};

pub trait WorkflowCommandBusPort: Send + Sync {
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
