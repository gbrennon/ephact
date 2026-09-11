#![allow(dead_code)]
use std::sync::{Arc, OnceLock};

use ephact::{
    application::{
        dtos::{requests::ExecuteActionRequest, responses::ExecuteActionResponse},
        ports::{
            inbound::ExecuteActionPort,
            outbound::{command_bus_port::CommandBusPort, container_port::ContainerPort},
        },
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
};

/// Routes dispatched action commands to a bound action executor, so a
/// composite action nesting another action exercises the real recursion the
/// command bus provides in production.
#[derive(Clone, Default)]
pub struct FakeActionRoutingCommandBus {
    executor: Arc<OnceLock<Arc<dyn ExecuteActionPort>>>,
}

impl FakeActionRoutingCommandBus {
    pub fn new() -> Self {
        Self {
            executor: Arc::new(OnceLock::new()),
        }
    }

    pub fn bind(&self, executor: Arc<dyn ExecuteActionPort>) {
        assert!(
            self.executor.set(executor).is_ok(),
            "executor already bound"
        );
    }
}

impl<'a> CommandBusPort<ExecuteActionCommand<'a, dyn ContainerPort>>
    for FakeActionRoutingCommandBus
{
    type Response = ExecuteActionResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        cmd: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        let executor = self
            .executor
            .get()
            .ok_or_else(|| StepError::new("no action executor bound"))?;
        let (action_ref, step, repo_path, env, context, container) = cmd.into_parts();
        executor.execute(ExecuteActionRequest::new(
            action_ref, step, repo_path, env, context, container,
        ))
    }
}
