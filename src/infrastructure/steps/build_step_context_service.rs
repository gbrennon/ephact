use crate::application::dtos::requests::BuildStepContextRequest;
use crate::application::ports::outbound::build_step_context_port::BuildStepContextPort;
use crate::domain::services::evaluation_context_mapper::EvaluationContextMapper;
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
    fn execute(&self, request: BuildStepContextRequest) -> EvaluationContext {
        let context =
            EvaluationContextMapper::from_parts(request.context().to_vec()).unwrap_or_default();
        let env = ContextValue::mapping(
            request
                .env()
                .iter()
                .map(|(key, value)| (key.clone(), ContextValue::text(value.clone()))),
        );
        context.with_env(env)
    }
}
