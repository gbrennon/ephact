use std::collections::HashMap;

use ephact::{
    application::{
        dtos::requests::BuildStepContextRequest,
        ports::outbound::step_context_builder_port::StepContextBuilderPort,
    },
    domain::value_objects::{ContextValue, EvaluationContext},
    infrastructure::steps::build_step_context_service::BuildStepContextService,
};

#[test]
fn execute_mirrors_the_environment_into_the_env_context() {
    let mut env = HashMap::new();
    env.insert("MODE".to_string(), "staging".to_string());

    let context = BuildStepContextService::new().build(BuildStepContextRequest::new(
        EvaluationContext::new(),
        env.clone(),
    ));

    assert_eq!(
        context.get("env").unwrap().property("MODE"),
        Some(&ContextValue::text("staging"))
    );
}

#[test]
fn execute_carries_every_other_context_field_over_unchanged() {
    let source = EvaluationContext::new()
        .with_root("secrets", ContextValue::text("secrets"))
        .with_root("github", ContextValue::text("github"))
        .with_root("runner", ContextValue::text("runner"))
        .with_root("inputs", ContextValue::text("inputs"));

    let context = BuildStepContextService::new()
        .build(BuildStepContextRequest::new(source.clone(), HashMap::new()));

    assert_eq!(context.get("secrets"), source.get("secrets"));
    assert_eq!(context.get("github"), source.get("github"));
    assert_eq!(context.get("runner"), source.get("runner"));
    assert_eq!(context.get("inputs"), source.get("inputs"));
}
