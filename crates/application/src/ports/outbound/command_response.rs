use crate::dtos::responses::{
    ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse, WorkflowExecutionResponse,
};

#[derive(Debug)]
pub enum CommandResponse {
    Workflow(WorkflowExecutionResponse),
    Job(JobExecutionResponse),
    Step(Box<ExecutedStepResponse>),
    Action(ExecuteActionResponse),
}
