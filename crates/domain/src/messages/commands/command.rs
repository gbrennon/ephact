use crate::messages::{
    Message,
    commands::{
        ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
    },
};

#[derive(Debug, Clone)]
pub enum Command<C: ?Sized + Send + Sync> {
    ExecuteAction(Box<ExecuteActionCommand<C>>),
    ExecuteJob(Box<ExecuteJobCommand>),
    ExecuteStep(Box<ExecuteStepCommand<C>>),
    ExecuteWorkflow(Box<ExecuteWorkflowCommand>),
}

impl<C: ?Sized + Send + Sync> Message for Command<C> {}
