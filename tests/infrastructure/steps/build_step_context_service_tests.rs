use ephact::{
    application::ports::outbound::build_step_context_port::BuildStepContextPort,
    infrastructure::steps::build_step_context_service::BuildStepContextService,
};
use std::collections::HashMap;

use ephact::{application::dtos::BuildStepContextRequest, domain::expression::EvalContext};
use serde_json::Value;

#[test]
fn execute_mirrors_the_environment_into_the_env_context() {
    let mut env = HashMap::new();
    env.insert("MODE".to_string(), "staging".to_string());

    let context = BuildStepContextService::new().execute(BuildStepContextRequest::new(&EvalContext::new(), &env));

    assert_eq!(context.env()["MODE"], "staging");
}

#[test]
fn execute_carries_every_other_context_field_over_unchanged() {
    let source = EvalContext::new()
        .with_secrets(Value::String("secrets".into()))
        .with_github(Value::String("github".into()))
        .with_runner(Value::String("runner".into()))
        .with_inputs(Value::String("inputs".into()));

    let context = BuildStepContextService::new().execute(BuildStepContextRequest::new(&source, &HashMap::new()));

    assert_eq!(context.secrets(), source.secrets());
    assert_eq!(context.github(), source.github());
    assert_eq!(context.runner(), source.runner());
    assert_eq!(context.inputs(), source.inputs());
}
