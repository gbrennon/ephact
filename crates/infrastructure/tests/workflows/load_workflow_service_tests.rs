use ephact::{
    application::{
        dtos::requests::LoadWorkflowRequest,
        ports::outbound::workflow_loader_port::WorkflowLoaderPort,
    },
    domain::value_objects::TriggerKind,
    infrastructure::workflows::load_workflow_service::LoadWorkflowService,
};

const VALID_WORKFLOW: &str = "name: Ci\non: push\nenv:\n  MODE: staging\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo hi\n";
const WOODPECKER_WORKFLOW: &str = r"name: Demo
when:
  - event: pull_request
steps:
  - name: verify
    image: alpine:3.20
    commands:
      - printf demo
";

#[test]
fn load_parses_valid_workflow_content() {
    let workflow = LoadWorkflowService::new()
        .load(LoadWorkflowRequest::new(
            VALID_WORKFLOW.to_string(),
            "ci.yml".to_string(),
        ))
        .unwrap();

    assert_eq!(workflow.name(), Some("Ci"));
    assert_eq!(workflow.file(), Some("ci.yml"));
    assert_eq!(
        workflow.env().get("MODE").map(String::as_str),
        Some("staging")
    );
    assert!(workflow.jobs().contains_key("build"));
}

#[test]
fn load_parses_woodpecker_pipeline_content() {
    let workflow = LoadWorkflowService::new()
        .load(LoadWorkflowRequest::new(
            WOODPECKER_WORKFLOW.to_string(),
            "demo.yml".to_string(),
        ))
        .unwrap();

    assert_eq!(workflow.name(), Some("Demo"));
    assert_eq!(workflow.file(), Some("demo.yml"));
    assert!(workflow.jobs().contains_key("step-0"));
    assert!(workflow.triggers_on(TriggerKind::PullRequest));
}

#[test]
fn load_errors_for_content_that_is_not_a_workflow_document() {
    let result = LoadWorkflowService::new().load(LoadWorkflowRequest::new(
        "- push\n- pull_request\n".to_string(),
        "ci.yml".to_string(),
    ));

    assert!(result.is_err());
}

#[test]
fn load_errors_for_malformed_yaml() {
    let result = LoadWorkflowService::new().load(LoadWorkflowRequest::new(
        "name: [unterminated\n".to_string(),
        "ci.yml".to_string(),
    ));

    assert!(result.is_err());
}

#[test]
fn load_uses_source_filename_for_content_only_workflows() {
    let workflow = LoadWorkflowService::new()
        .load(LoadWorkflowRequest::new(
            "on: push\njobs: {}\n".to_string(),
            "ci.yml".to_string(),
        ))
        .unwrap();

    assert_eq!(workflow.name(), None);
    assert_eq!(workflow.file(), Some("ci.yml"));
}
