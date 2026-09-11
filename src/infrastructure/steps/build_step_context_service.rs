use crate::application::dtos::requests::BuildStepContextRequest;
use crate::application::ports::outbound::build_step_context_port::BuildStepContextPort;
use crate::domain::value_objects::ContextValue;
use crate::domain::value_objects::EvaluationContext;

/// Service that mirrors a step's environment into the `env` expression context.
pub struct BuildStepContextService;

impl BuildStepContextService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BuildStepContextService {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildStepContextPort for BuildStepContextService {
    fn execute(&self, request: BuildStepContextRequest<'_>) -> EvaluationContext {
        let env = ContextValue::mapping(
            request
                .env()
                .iter()
                .map(|(key, value)| (key.clone(), ContextValue::text(value.clone()))),
        );
        request.context().clone().with_env(env)
    }
}
