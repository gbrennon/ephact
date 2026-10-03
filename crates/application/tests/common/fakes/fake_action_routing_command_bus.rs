use std::sync::{Arc, OnceLock};

use ephact::{
    application::{
        dtos::requests::{
            ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
        },
        ports::outbound::{ActionCommandPublisherPort, ContainerPort, StepTextCodecPort},
    },
    domain::{errors::StepError, messages::commands::ExecuteActionPayload},
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

impl ActionCommandPublisherPort for FakeActionRoutingCommandBus {
    fn publish(
        &self,
        command: ExecuteActionPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ephact::application::dtos::responses::ExecuteActionResponse, StepError> {
        let factory = self
            .executor_factory
            .get()
            .ok_or_else(|| StepError::new("no action executor bound"))?;
        let (action_ref, step, repo_path, env, context) = command.into_parts();
        let executor = factory(container);
        let encoded_step = JsonStepTextCodec.encode(&step)?;
        executor
            .execute(ExecuteActionRequest::new(ExecuteActionRequestInput::new(
                action_ref,
                encoded_step,
                ExecuteActionExecutionInput::new(repo_path, env, context),
            )))
            .map_err(|error| StepError::new(error.to_string()))
    }
}
