use std::env;
use std::path::PathBuf;

use ephact::presentation::handlers::ListActionsHandler;

use crate::common::fakes::fake_list_actions_port::FakeListActionsPort;

#[test]
fn handle_returns_actions_from_port() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let port = FakeListActionsPort::with_actions(actions.clone());

    let response =
        ListActionsHandler::handle(&port, env::current_dir().expect("current directory"))
            .expect("repository should be valid");

    assert_eq!(response.actions(), actions.as_slice());
}

#[test]
fn handle_returns_error_on_invalid_repo_path() {
    let port = FakeListActionsPort::new();
    let invalid_path = PathBuf::from("/definitely/not/a/repository");

    let result = ListActionsHandler::handle(&port, invalid_path);

    assert!(result.is_err());
}

#[test]
fn handle_returns_error_when_port_fails() {
    let port = FakeListActionsPort::failing("workflow source failure");

    let result = ListActionsHandler::handle(&port, env::current_dir().expect("current directory"));

    assert!(result.is_err());
}

#[test]
fn render_keeps_final_reference_segment_of_each_action() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "./.forgejo/actions/build".to_string(),
    ];
    let port = FakeListActionsPort::with_actions(actions);

    let response =
        ListActionsHandler::handle(&port, env::current_dir().expect("current directory"))
            .expect("repository should be valid");
    let rendered = ListActionsHandler::render(&response);

    assert_eq!(rendered, "checkout@v4\nbuild");
}
