use crate::messages::{
    Message,
    commands::{
        ExecuteActionPayload, ExecuteJobPayload, ExecuteStepPayload, ExecuteWorkflowPayload,
    },
};

#[derive(Debug, Clone)]
pub enum Command {
    ExecuteAction(Box<ExecuteActionPayload>),
    ExecuteJob(Box<ExecuteJobPayload>),
    ExecuteStep(Box<ExecuteStepPayload>),
    ExecuteWorkflow(Box<ExecuteWorkflowPayload>),
}

impl Message for Command {}
