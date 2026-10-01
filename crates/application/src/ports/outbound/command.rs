use super::container_port::ContainerPort;
use crate::domain::messages::commands::{
    ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
};

#[derive(Debug, Clone)]
pub enum Command {
    Workflow(ExecuteWorkflowCommand),
    Job(Box<ExecuteJobCommand>),
    Step(ExecuteStepCommand<dyn ContainerPort>),
    Action(ExecuteActionCommand<dyn ContainerPort>),
}
