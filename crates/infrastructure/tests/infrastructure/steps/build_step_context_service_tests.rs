#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ephact::{
        application::{
            dtos::requests::BuildStepContextRequest,
            ports::outbound::step_context_builder_port::StepContextBuilderPort,
        },
        domain::{
            services::evaluation_context_mapper::EvaluationContextMapper,
            value_objects::{ContextValue, EvaluationContext},
        },
        infrastructure::steps::build_step_context_service::BuildStepContextService,
    };

    #[test]
    fn execute_mirrors_the_environment_into_the_env_context() {
        let mut env = HashMap::new();
        env.insert("MODE".to_string(), "staging".to_string());

        let context = BuildStepContextService::new().build(BuildStepContextRequest::new(
            EvaluationContextMapper::to_parts(&EvaluationContext::new()),
            env.clone(),
        ));

        assert_eq!(
            context.env().property("MODE"),
            Some(&ContextValue::text("staging"))
        );
    }

    #[test]
    fn execute_carries_every_other_context_field_over_unchanged() {
        let source = EvaluationContext::new()
            .with_secrets(ContextValue::text("secrets"))
            .with_github(ContextValue::text("github"))
            .with_runner(ContextValue::text("runner"))
            .with_inputs(ContextValue::text("inputs"));

        let context = BuildStepContextService::new().build(BuildStepContextRequest::new(
            EvaluationContextMapper::to_parts(&source),
            HashMap::new(),
        ));

        assert_eq!(context.secrets(), source.secrets());
        assert_eq!(context.github(), source.github());
        assert_eq!(context.runner(), source.runner());
        assert_eq!(context.inputs(), source.inputs());
    }
}
