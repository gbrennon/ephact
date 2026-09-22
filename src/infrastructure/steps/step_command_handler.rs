use crate::{
    application::{
        dtos::{requests::ExecuteStepRequest, responses::ExecutedStepResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::{
        errors::StepError,
        messages::commands::ExecuteStepCommand,
        services::{evaluation_context_mapper::EvaluationContextMapper, step_factory::StepFactory},
    },
    infrastructure::steps::ExecuteStepFactory,
};

/// Infrastructure command handler that processes `ExecuteStepCommand`.
pub struct StepCommandHandler {
    executor_factory: ExecuteStepFactory,
}

impl StepCommandHandler {
    pub fn new(executor_factory: ExecuteStepFactory) -> Self {
        Self { executor_factory }
    }

    pub fn handle(
        &self,
        cmd: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        let (step, env, context, container, repo_path) = cmd.into_parts();
        let req = ExecuteStepRequest::new(
            StepFactory::to_text(&step)?,
            EvaluationContextMapper::to_parts(&context),
            repo_path,
            env,
        );
        let executor = (self.executor_factory)(container);
        executor
            .execute(req)
            .map_err(|error| StepError::new(error.to_string()))
    }
}
