use ephact::{domain::entities::Job, infrastructure::workflows::woodpecker::WoodpeckerStepYaml};

const STEP: &str = r#"
name: test
image: rust
commands:
  - cargo build
  - cargo test
"#;

struct WoodpeckerStepYamlTest;

impl WoodpeckerStepYamlTest {
    fn job_from(yaml: &str, needs: Vec<String>) -> Job {
        WoodpeckerStepYaml::parse(yaml)
            .unwrap()
            .into_domain_job(needs)
    }
}

#[test]
fn the_step_name_becomes_the_job_name() {
    assert_eq!(
        WoodpeckerStepYamlTest::job_from(STEP, Vec::new()).name(),
        Some("test")
    );
}

#[test]
fn the_step_image_becomes_the_job_container() {
    let job = WoodpeckerStepYamlTest::job_from(STEP, Vec::new());

    assert_eq!(job.container().map(|image| image.image()), Some("rust"));
}

#[test]
fn the_commands_join_into_a_single_run_script() {
    let job = WoodpeckerStepYamlTest::job_from(STEP, Vec::new());
    let step = job.steps().first().expect("run step");

    assert_eq!(step.run(), Some("cargo build\ncargo test"));
}

#[test]
fn the_supplied_dependencies_become_the_job_needs() {
    let job = WoodpeckerStepYamlTest::job_from(STEP, vec!["step-0".to_owned()]);

    assert_eq!(job.needs(), ["step-0"]);
}

#[test]
fn a_step_without_an_image_has_no_container() {
    let job = WoodpeckerStepYamlTest::job_from("name: noop\ncommands: [echo hi]", Vec::new());

    assert!(job.container().is_none());
}

#[test]
fn deferred_variables_become_shell_variables() {
    let job = WoodpeckerStepYamlTest::job_from(
        r#"
name: expansion
commands:
  - |
      VALUE="expected"
      printf '%s' "$${VALUE}"
"#,
        Vec::new(),
    );
    let step = job.steps().first().expect("run step");

    assert_eq!(
        step.run(),
        Some("VALUE=\"expected\"\nprintf '%s' \"${VALUE}\"\n")
    );
}

#[test]
fn a_lone_double_dollar_collapses_to_a_single_dollar() {
    let job = WoodpeckerStepYamlTest::job_from(
        r#"
name: process
commands:
  - printf '%s %s' "$${VALUE}" "$$"
"#,
        Vec::new(),
    );
    let step = job.steps().first().expect("run step");

    assert_eq!(step.run(), Some("printf '%s %s' \"${VALUE}\" \"$\""));
}
