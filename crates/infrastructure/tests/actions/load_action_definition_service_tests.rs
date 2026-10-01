use std::fs;

use ephact::{
    application::{
        dtos::requests::LoadActionDefinitionRequest,
        ports::outbound::action_definition_loader_port::ActionDefinitionLoaderPort,
    },
    domain::value_objects::ActionRuntime,
    infrastructure::actions::preparation::load_action_definition_service::LoadActionDefinitionService,
};

const COMPOSITE: &str =
    "name: Greet\nruns:\n  using: composite\n  steps:\n    - run: echo hi\n      shell: bash\n";
const COMPOSITE_WITHOUT_NAME: &str =
    "runs:\n  using: composite\n  steps:\n    - run: echo hi\n      shell: bash\n";
const COMPOSITE_WITH_BLANK_NAME: &str =
    "name: \"  \"\nruns:\n  using: composite\n  steps:\n    - run: echo hi\n      shell: bash\n";

#[test]
fn execute_loads_action_yml() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), COMPOSITE).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();

    assert_eq!(definition.name(), "Greet");
    assert!(matches!(definition.runs(), ActionRuntime::Composite { .. }));
}

#[test]
fn execute_falls_back_to_action_yaml() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yaml"), COMPOSITE).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();

    assert_eq!(definition.name(), "Greet");
}

#[test]
fn execute_uses_action_yml_filename_when_name_is_missing() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), COMPOSITE_WITHOUT_NAME).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();

    assert_eq!(definition.name(), "action.yml");
}

#[test]
fn execute_uses_action_yaml_filename_when_name_is_missing() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yaml"), COMPOSITE_WITHOUT_NAME).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();

    assert_eq!(definition.name(), "action.yaml");
}

#[test]
fn execute_uses_action_yml_filename_when_name_is_blank() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), COMPOSITE_WITH_BLANK_NAME).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();

    assert_eq!(definition.name(), "action.yml");
}

#[test]
fn execute_uses_action_yaml_filename_when_name_is_blank() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yaml"), COMPOSITE_WITH_BLANK_NAME).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();

    assert_eq!(definition.name(), "action.yaml");
}

#[test]
fn execute_errors_when_no_definition_is_present() {
    let tmp = tempfile::tempdir().unwrap();

    let error = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap_err();

    assert_eq!(
        error.message(),
        format!("action.yml not found in {}", tmp.path().display())
    );
}

#[test]
fn execute_errors_on_malformed_yaml() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), "name: [unterminated\n").unwrap();

    let error = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap_err();

    assert!(
        error.message().starts_with("failed to parse "),
        "{}",
        error.message()
    );
}
