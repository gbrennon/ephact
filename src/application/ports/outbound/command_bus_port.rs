use std::error::Error;

use crate::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
            WorkflowExecutionResponse,
        },
        ports::outbound::container_port::ContainerPort,
    },
    domain::{
        errors::StepError,
        messages::commands::{
            Command, ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand,
            ExecuteWorkflowCommand,
        },
    },
};

/// Outbound port representing a generic command bus.
///
/// Dispatches commands to their corresponding command handlers.
pub trait CommandBusPort<C: Command>: Send + Sync {
    type Response;
    type Error;

    /// Dispatches a command and returns the handler's outcome.
    fn dispatch(&self, command: C) -> Result<Self::Response, Self::Error>;
}

pub type WorkflowCommandBusPort = dyn CommandBusPort<
        ExecuteWorkflowCommand,
        Response = WorkflowExecutionResponse,
        Error = Box<dyn Error>,
    >;

pub type JobCommandBusPort =
    dyn CommandBusPort<ExecuteJobCommand, Response = JobExecutionResponse, Error = Box<dyn Error>>;

pub type StepCommandBusPort = dyn for<'a> CommandBusPort<
        ExecuteStepCommand<'a, dyn ContainerPort>,
        Response = ExecutedStepResponse,
        Error = StepError,
    >;

pub type ActionCommandBusPort = dyn for<'a> CommandBusPort<
        ExecuteActionCommand<'a, dyn ContainerPort>,
        Response = ExecuteActionResponse,
        Error = StepError,
    >;
