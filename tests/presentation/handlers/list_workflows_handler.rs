use std::env;
use std::path::PathBuf;

use ephact::application::dtos::responses::WorkflowListItemResponse;
use ephact::presentation::handlers::ListWorkflowsHandler;

use crate::common::fakes::fake_list_workflows_port::FakeListWorkflowsPort;

#[test]
fn handle_returns_workflows_from_port() {
    let workflows = vec![WorkflowListItemResponse::new(
        Some("CI".into()),
        Some("ci.yml".into()),
        vec!["push".into()],
    )];
    let port = FakeListWorkflowsPort::with_workflows(workflows.clone());

    let response =
        ListWorkflowsHandler::handle(&port, env::current_dir().expect("current directory"))
            .expect("repository should be valid");

    assert_eq!(response.workflows(), workflows);
}

#[test]
fn handle_returns_error_on_invalid_repo_path() {
    let port = FakeListWorkflowsPort::new();
    let invalid_path = PathBuf::from("/definitely/not/a/repository");

    let result = ListWorkflowsHandler::handle(&port, invalid_path);

    assert!(result.is_err());
}

#[test]
fn handle_returns_error_when_port_fails() {
    let port = FakeListWorkflowsPort::failing("workflow source failure");

    let result =
        ListWorkflowsHandler::handle(&port, env::current_dir().expect("current directory"));

    assert!(result.is_err());
}

#[test]
fn render_joins_workflow_names_with_placeholder_for_unnamed() {
    let workflows = vec![
        WorkflowListItemResponse::new(Some("CI".into()), Some("ci.yml".into()), vec![]),
        WorkflowListItemResponse::new(None, Some("mystery.yml".into()), vec![]),
    ];
    let port = FakeListWorkflowsPort::with_workflows(workflows);

    let response =
        ListWorkflowsHandler::handle(&port, env::current_dir().expect("current directory"))
            .expect("repository should be valid");
    let rendered = ListWorkflowsHandler::render(&response);

    assert_eq!(rendered, "CI\nUnnamed workflow");
}
