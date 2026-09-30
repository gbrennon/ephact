use ephact::infrastructure::workflows::workflow_document::WorkflowDocument;

const ACTIONS_CONTENT: &str = r#"name: CI
on: push
jobs:
  build:
    runs-on: linux
    steps:
      - run: echo ok
"#;

const WOODPECKER_CONTENT: &str = r#"name: CI
when:
  - event: push
steps:
  - name: build
    image: alpine
    commands:
      - echo ok
"#;

#[test]
fn parse_selects_actions_for_jobs_documents() {
    let document = WorkflowDocument::parse(
        r#"name: CI
on: push
jobs:
  build:
    runs-on: linux
    steps:
      - run: echo ok
"#,
    )
    .expect("Actions workflow should parse");

    assert!(matches!(document, WorkflowDocument::Actions(_)));
}

#[test]
fn parse_selects_woodpecker_for_when_and_steps_documents() {
    let document = WorkflowDocument::parse(
        r#"name: CI
when:
  - event: push
steps:
  - name: build
    image: alpine
    commands:
      - echo ok
"#,
    )
    .expect("Woodpecker pipeline should parse");

    assert!(matches!(document, WorkflowDocument::Woodpecker(_)));
}

#[test]
fn actions_document_returns_workflow_name() {
    let document = WorkflowDocument::parse(ACTIONS_CONTENT).expect("should parse");
    assert_eq!(document.name(), Some("CI".to_owned()));
}

#[test]
fn woodpecker_document_returns_workflow_name() {
    let document = WorkflowDocument::parse(WOODPECKER_CONTENT).expect("should parse");
    assert_eq!(document.name(), Some("CI".to_owned()));
}

#[test]
fn actions_document_without_name_returns_none() {
    let content = r#"on: push
jobs:
  build:
    runs-on: linux
    steps:
      - run: echo ok
"#;
    let document = WorkflowDocument::parse(content).expect("should parse");
    assert_eq!(document.name(), None);
}

#[test]
fn actions_document_returns_trigger_names() {
    let document = WorkflowDocument::parse(ACTIONS_CONTENT).expect("should parse");
    assert_eq!(document.trigger_names(), vec!["push"]);
}

#[test]
fn woodpecker_document_returns_trigger_names() {
    let document = WorkflowDocument::parse(WOODPECKER_CONTENT).expect("should parse");
    assert_eq!(document.trigger_names(), vec!["push"]);
}

#[test]
fn actions_document_into_domain_returns_correct_workflow() {
    let domain = WorkflowDocument::parse(ACTIONS_CONTENT)
        .expect("should parse")
        .into_domain();
    assert_eq!(domain.name(), Some("CI"));
    assert!(domain.jobs().contains_key("build"));
}

#[test]
fn woodpecker_document_into_domain_returns_correct_workflow() {
    let domain = WorkflowDocument::parse(WOODPECKER_CONTENT)
        .expect("should parse")
        .into_domain();
    assert_eq!(domain.name(), Some("CI"));
    assert!(domain.jobs().contains_key("step-0"));
}

/// Regression: a nested job `name` field must not be returned as the workflow name.
#[test]
fn nested_job_name_is_not_returned_as_workflow_name() {
    let content = r#"on: push
jobs:
  build:
    name: My Build Job
    runs-on: linux
    steps:
      - run: echo ok
"#;
    let document = WorkflowDocument::parse(content).expect("should parse");
    assert_eq!(
        document.name(),
        None,
        "nested job name must not be treated as the workflow name"
    );
}
