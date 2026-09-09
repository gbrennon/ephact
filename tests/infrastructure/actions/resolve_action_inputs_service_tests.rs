use ephact::{
    application::dtos::ResolveActionInputsRequest,
    domain::workflow::{ActionDefinition, Step},
};
use ephact::{
    application::ports::outbound::resolve_action_inputs_port::ResolveActionInputsPort,
    infrastructure::actions::resolve_action_inputs_service::ResolveActionInputsService,
};

const WITH_DEFAULTS: &str = "name: Deploy\ninputs:\n  mode:\n    description: target\n    default: production\n  token:\n    description: secret\nruns:\n  using: composite\n  steps: []\n";

fn definition() -> ActionDefinition {
    serde_yaml::from_str(WITH_DEFAULTS).unwrap()
}

fn step(yaml: &str) -> Step {
    serde_yaml::from_str(yaml).unwrap()
}

#[test]
fn execute_returns_the_declared_defaults() {
    let inputs = ResolveActionInputsService::new()
        .execute(ResolveActionInputsRequest::new(
            &definition(),
            &step("uses: ./actions/deploy\n"),
        ))
        .unwrap();

    assert_eq!(inputs.get("mode").map(String::as_str), Some("production"));
}

#[test]
fn execute_lets_with_override_a_default() {
    let inputs = ResolveActionInputsService::new()
        .execute(ResolveActionInputsRequest::new(
            &definition(),
            &step("uses: ./actions/deploy\nwith:\n  mode: staging\n"),
        ))
        .unwrap();

    assert_eq!(inputs.get("mode").map(String::as_str), Some("staging"));
}

#[test]
fn execute_omits_an_input_with_neither_default_nor_with() {
    let inputs = ResolveActionInputsService::new()
        .execute(ResolveActionInputsRequest::new(
            &definition(),
            &step("uses: ./actions/deploy\n"),
        ))
        .unwrap();

    assert!(!inputs.contains_key("token"));
}

#[test]
fn execute_passes_undeclared_with_keys_through() {
    let inputs = ResolveActionInputsService::new()
        .execute(ResolveActionInputsRequest::new(
            &definition(),
            &step("uses: ./actions/deploy\nwith:\n  extra: value\n"),
        ))
        .unwrap();

    assert_eq!(inputs.get("extra").map(String::as_str), Some("value"));
}

#[test]
fn execute_reports_a_missing_required_input() {
    let required_definition: ActionDefinition = serde_yaml::from_str(
        "name: Deploy\ninputs:\n  token:\n    required: true\nruns:\n  using: composite\n  steps: []\n",
    )
    .unwrap();

    let error = ResolveActionInputsService::new()
        .execute(ResolveActionInputsRequest::new(
            &required_definition,
            &step("uses: ./actions/deploy\n"),
        ))
        .unwrap_err();

    assert!(error.message().contains("token"));
}
