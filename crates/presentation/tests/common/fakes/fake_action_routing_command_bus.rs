use std::sync::{Arc, OnceLock};

use ephact::{
    application::{
        dtos::requests::{
            ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
        },
        ports::outbound::{
            Command, CommandError, CommandPublisherPort, CommandResponse, StepTextCodecPort,
        },
    },
    domain::errors::StepError,
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

impl CommandPublisherPort for FakeActionRoutingCommandBus {
    fn publish(&self, command: Command) -> Result<CommandResponse, CommandError> {
        let Command::Action(command) = command else {
            return Err(CommandError::Transport(
                "action routing fake received a non-action command".to_string(),
            ));
        };
        let factory = self
            .executor_factory
            .get()
            .ok_or_else(|| CommandError::Step(StepError::new("no action executor bound")))?;
        let (action_ref, step, repo_path, env, context, container) = command.into_parts();
        let executor = factory(container);
        executor
            .execute(ExecuteActionRequest::new(ExecuteActionRequestInput::new(
                action_ref,
                JsonStepTextCodec
                    .encode(&step)
                    .map_err(CommandError::Step)?,
                ExecuteActionExecutionInput::new(repo_path, env, context),
            )))
            .map(CommandResponse::Action)
            .map_err(|error| CommandError::Step(StepError::new(error.to_string())))
    }
}
