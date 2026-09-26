use ephact::{
    domain::{entities::Job, value_objects::ContextValue},
    infrastructure::workflows::yaml::JobYaml,
};

fn job_from(yaml: &str) -> Job {
    serde_yaml::from_str::<JobYaml>(yaml).unwrap().into_domain()
}

#[test]
fn a_minimal_job_keeps_its_runner_and_steps() {
    let job = job_from("runs-on: ubuntu-latest\nsteps:\n  - run: echo hello\n");

    assert_eq!(job.runs_on(), Some("ubuntu-latest"));
    assert_eq!(job.steps().len(), 1);
}

#[test]
fn a_single_needs_entry_becomes_a_one_element_dependency_list() {
    let job = job_from("needs: build\n");

    assert_eq!(job.needs(), &["build".to_string()]);
}

#[test]
fn a_needs_sequence_keeps_every_dependency() {
    let job = job_from("needs: [build, lint]\nif: github.ref == 'refs/heads/main'\n");

    assert_eq!(job.needs(), &["build".to_string(), "lint".to_string()]);
    assert_eq!(job.r#if(), Some("github.ref == 'refs/heads/main'"));
}

#[test]
fn a_job_without_needs_has_an_empty_dependency_list() {
    let job = job_from("runs-on: ubuntu-latest\n");

    assert_eq!(job.needs(), Vec::<String>::new().as_slice());
}

#[test]
fn a_container_entry_reaches_the_domain_job() {
    let yaml = "runs-on: ubuntu-latest\ncontainer:\n  image: node:18\n  env:\n    NODE_ENV: test\n";

    let job = job_from(yaml);

    let container = job.container().unwrap();
    assert_eq!(container.image(), "node:18");
    assert_eq!(
        container.env().get("NODE_ENV").map(String::as_str),
        Some("test")
    );
}

#[test]
fn a_timeout_entry_reaches_the_domain_job() {
    let job = job_from("runs-on: ubuntu-latest\ntimeout-minutes: 30\n");

    assert_eq!(job.timeout_minutes(), Some(30.0));
}

#[test]
fn dynamic_with_and_secrets_entries_become_context_values() {
    let yaml = "with:\n  mode: staging\n  retries: 3\nsecrets: inherit\n";

    let job = job_from(yaml);

    assert_eq!(
        job.with().and_then(|with| with.property("mode")),
        Some(&ContextValue::text("staging"))
    );
    assert_eq!(
        job.with().and_then(|with| with.property("retries")),
        Some(&ContextValue::Integer(3))
    );
    assert_eq!(job.secrets(), Some(&ContextValue::text("inherit")));
}
