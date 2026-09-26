use std::sync::{Arc, OnceLock};

use ephact::{
    application::{
        dtos::{
            requests::{
                ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
            },
            responses::ExecuteActionResponse,
        },
        ports::outbound::{
            StepTextCodecPort, action_command_bus_port::ActionCommandBusPort,
            container_port::ContainerPort,
        },
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
    infrastructure::{actions::ExecuteActionFactory, steps::JsonStepTextCodec},
};

#[derive(Clone, Default)]
pub struct FakeActionRoutingCommandBus {
    executor_factory: Arc<OnceLock<Arc<ExecuteActionFactory>>>,
}

impl FakeActionRoutingCommandBus {
    pub fn new() -> Self {
        Self {
            executor_factory: Arc::new(OnceLock::new()),
        }
    }

    pub fn bind(&self, executor_factory: Arc<ExecuteActionFactory>) {
        assert!(
            self.executor_factory.set(executor_factory).is_ok(),
            "executor already bound"
        );
    }
}
impl ActionCommandBusPort for FakeActionRoutingCommandBus {
    fn dispatch(
        &self,
        cmd: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let factory = self
            .executor_factory
            .get()
            .ok_or_else(|| StepError::new("no action executor bound"))?;
        let (action_ref, step, repo_path, env, context, container) = cmd.into_parts();
        let executor = factory(container);
        executor
            .execute(ExecuteActionRequest::new(ExecuteActionRequestInput::new(
                action_ref,
                JsonStepTextCodec.encode(&step).unwrap(),
                ExecuteActionExecutionInput::new(repo_path, env, context),
            )))
            .map_err(|error| StepError::new(error.to_string()))
    }
}
