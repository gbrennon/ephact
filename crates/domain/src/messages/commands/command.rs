use crate::messages::{
    Message,
    commands::{
        ExecuteActionPayload, ExecuteJobPayload, ExecuteStepPayload, ExecuteWorkflowPayload,
    },
};

/// A command representing an operation to execute.
#[derive(Debug, Clone)]
pub enum Command {
    /// Executes an action.
    ExecuteAction(Box<ExecuteActionPayload>),
    /// Executes a job.
    ExecuteJob(Box<ExecuteJobPayload>),
    /// Executes a step.
    ExecuteStep(Box<ExecuteStepPayload>),
    /// Executes a workflow.
    ExecuteWorkflow(Box<ExecuteWorkflowPayload>),
}

impl Message for Command {}
