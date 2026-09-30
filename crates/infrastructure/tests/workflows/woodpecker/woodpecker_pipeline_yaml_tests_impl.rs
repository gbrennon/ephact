use ephact::{
    domain::{aggregates::Workflow, value_objects::TriggerKind},
    infrastructure::workflows::woodpecker::WoodpeckerPipelineYaml,
};

const PIPELINE: &str = r#"
name: Woodpecker CI

when:
  - event: [push, pull_request]

steps:
  - name: build
    image: alpine
    commands:
      - echo one
      - echo two
  - name: test
    image: rust
    commands:
      - cargo test
"#;

fn workflow_from(yaml: &str) -> Workflow {
    WoodpeckerPipelineYaml::parse(yaml)
        .expect("pipeline parses")
        .into_domain()
}

#[test]
fn into_domain_keeps_the_pipeline_name() {
    assert_eq!(workflow_from(PIPELINE).name(), Some("Woodpecker CI"));
}

#[test]
fn the_when_conditions_become_domain_triggers() {
    let workflow = workflow_from(PIPELINE);

    assert!(workflow.triggers_on(TriggerKind::Push));
    assert!(workflow.triggers_on(TriggerKind::PullRequest));
}

#[test]
fn every_step_becomes_a_job() {
    assert_eq!(workflow_from(PIPELINE).jobs().len(), 2);
}

#[test]
fn the_first_step_job_has_no_dependencies() {
    let workflow = workflow_from(PIPELINE);
    let job = workflow.jobs().get("step-0").expect("first step job");

    assert!(job.needs().is_empty());
}

#[test]
fn later_step_jobs_depend_on_their_predecessor_to_preserve_order() {
    let workflow = workflow_from(PIPELINE);
    let job = workflow.jobs().get("step-1").expect("second step job");

    assert_eq!(job.needs(), ["step-0"]);
}

#[test]
fn an_empty_pipeline_parses_to_a_workflow_without_jobs() {
    assert!(workflow_from("steps: []").jobs().is_empty());
}
