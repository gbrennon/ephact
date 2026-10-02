//! Integration regression test for issue #265 (GitHub side).
//!
//! Reproduces the `ComChan` failure where loading `dtolnay/rust-toolchain`'s
//! composite `action.yml` aborted because its steps declared unquoted scalar
//! `env`/`continue-on-error` values (`RUSTUP_PERMIT_COPY_RENAME: 1`,
//! `continue-on-error: true`). Before the fix the loader errored with
//! `invalid type: integer 1, expected a string`, which cascaded into the
//! reported `cargo: command not found` (exit 127).

use std::fs;

use ephact::{
    application::{
        dtos::requests::LoadActionDefinitionRequest,
        ports::outbound::action_definition_loader_port::ActionDefinitionLoaderPort,
    },
    domain::value_objects::ActionRuntime,
    infrastructure::actions::preparation::load_action_definition_service::LoadActionDefinitionService,
};

/// A composite action whose steps carry unquoted scalar `env`/`continue-on-error`
/// values, mirroring the real `dtolnay/rust-toolchain` metadata.
const COMPOSITE_WITH_SCALAR_FIELDS: &str = r#"
name: rustup toolchain install
runs:
  using: composite
  steps:
    - name: install
      run: rustup toolchain install
      shell: bash
      env:
        RUSTUP_PERMIT_COPY_RENAME: 1
      continue-on-error: true
"#;

fn composite_steps(
    definition: &ephact::domain::value_objects::ActionDefinition,
) -> Vec<ephact::domain::entities::Step> {
    match definition.runs() {
        ActionRuntime::Composite { steps } => steps.clone(),
        other => panic!("expected a composite runtime, got {other:?}"),
    }
}

#[test]
fn a_composite_action_with_scalar_metadata_loads_from_disk() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), COMPOSITE_WITH_SCALAR_FIELDS).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .expect("composite action metadata parses");

    assert_eq!(definition.name(), "rustup toolchain install");
}

#[test]
fn an_unquoted_numeric_env_value_is_normalized_to_a_string() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), COMPOSITE_WITH_SCALAR_FIELDS).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();
    let steps = composite_steps(&definition);

    assert_eq!(
        steps[0]
            .env()
            .get("RUSTUP_PERMIT_COPY_RENAME")
            .map(String::as_str),
        Some("1")
    );
}

#[test]
fn an_unquoted_boolean_continue_on_error_is_normalized_to_a_string() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("action.yml"), COMPOSITE_WITH_SCALAR_FIELDS).unwrap();

    let definition = LoadActionDefinitionService::new()
        .load(LoadActionDefinitionRequest::new(tmp.path().to_path_buf()))
        .unwrap();
    let steps = composite_steps(&definition);

    assert_eq!(steps[0].continue_on_error(), Some("true"));
}
