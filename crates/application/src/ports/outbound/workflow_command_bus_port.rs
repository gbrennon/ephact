use crate::{
    domain::messages::commands::ExecuteWorkflowCommand, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

pub trait WorkflowCommandBusPort: Send + Sync {
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
