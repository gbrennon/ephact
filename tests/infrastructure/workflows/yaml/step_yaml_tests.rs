use ephact::{domain::entities::Step, infrastructure::workflows::yaml::StepYaml};

fn step_from(yaml: &str) -> Step {
    serde_yaml::from_str::<StepYaml>(yaml)
        .unwrap()
        .into_domain()
}

#[test]
fn a_run_entry_becomes_a_run_step() {
    let step = step_from("run: cargo test\n");

    assert!(step.is_run_step());
    assert_eq!(step.run(), Some("cargo test"));
}

#[test]
fn a_uses_entry_becomes_a_uses_step() {
    let step = step_from("uses: actions/checkout@v4\n");

    assert!(step.is_uses_step());
    assert_eq!(step.uses(), Some("actions/checkout@v4"));
}

#[test]
fn every_authored_field_reaches_the_domain_step() {
    let yaml = "id: test-step\nname: Run tests\nif: success()\nrun: cargo test\nshell: bash\nworking-directory: ./src\nenv:\n  RUST_LOG: debug\ncontinue-on-error: true\ntimeout-minutes: 10\n";

    let step = step_from(yaml);

    assert_eq!(step.id(), Some("test-step"));
    assert_eq!(step.name(), Some("Run tests"));
    assert_eq!(step.r#if(), Some("success()"));
    assert_eq!(step.run(), Some("cargo test"));
    assert_eq!(step.shell(), Some("bash"));
    assert_eq!(step.working_directory(), Some("./src"));
    assert_eq!(
        step.env().get("RUST_LOG").map(String::as_str),
        Some("debug")
    );
    assert_eq!(step.continue_on_error(), Some("true"));
    assert_eq!(step.timeout_minutes(), Some(10.0));
}

#[test]
fn action_inputs_reach_the_domain_step() {
    let step = step_from("uses: ./action\nwith:\n  mode: staging\n");

    assert_eq!(step.with().get("mode").map(String::as_str), Some("staging"));
}
