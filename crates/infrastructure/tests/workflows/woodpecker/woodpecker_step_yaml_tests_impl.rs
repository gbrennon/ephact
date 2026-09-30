use ephact::{domain::entities::Job, infrastructure::workflows::woodpecker::WoodpeckerStepYaml};

const STEP: &str = r#"
name: test
image: rust
commands:
  - cargo build
  - cargo test
"#;

fn job_from(yaml: &str, needs: Vec<String>) -> Job {
    serde_yaml::from_str::<WoodpeckerStepYaml>(yaml)
        .unwrap()
        .into_domain_job(needs)
}

#[test]
fn the_step_name_becomes_the_job_name() {
    assert_eq!(job_from(STEP, Vec::new()).name(), Some("test"));
}

#[test]
fn the_step_image_becomes_the_job_container() {
    let job = job_from(STEP, Vec::new());

    assert_eq!(job.container().map(|image| image.image()), Some("rust"));
}

#[test]
fn the_commands_join_into_a_single_run_script() {
    let job = job_from(STEP, Vec::new());
    let step = job.steps().first().expect("run step");

    assert_eq!(step.run(), Some("cargo build\ncargo test"));
}

#[test]
fn the_supplied_dependencies_become_the_job_needs() {
    let job = job_from(STEP, vec!["step-0".to_owned()]);

    assert_eq!(job.needs(), ["step-0"]);
}

#[test]
fn a_step_without_an_image_has_no_container() {
    let job = job_from("name: noop\ncommands: [echo hi]", Vec::new());

    assert!(job.container().is_none());
}
