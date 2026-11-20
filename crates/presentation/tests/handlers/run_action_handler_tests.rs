use std::{collections::HashMap, path::PathBuf};

use ephact::{
    application::{dtos::responses::ExecuteActionResponse, ports::outbound::StepTextCodecPort},
    domain::value_objects::{ContextValue, EvaluationContext},
    infrastructure::steps::JsonStepTextCodec,
    presentation::handlers::RunActionHandler,
};

#[path = "recording_run_action_port.rs"]
mod recording_run_action_port;

use recording_run_action_port::RecordingRunActionPort;

#[tokio::test]
async fn handle_delegates_selected_action_without_workflow_selection() {
    let response = ExecuteActionResponse::new(0, "done".to_owned(), String::new());
    let port = RecordingRunActionPort::new(response.clone());
    let repository_path = PathBuf::from("/repo");

    let actual = RunActionHandler::handle(
        &port,
        &JsonStepTextCodec,
        "actions/checkout@v4".to_owned(),
        vec![("ref".to_owned(), "main".to_owned())],
        repository_path.clone(),
    )
    .await
    .expect("action should run");

    assert_eq!(actual, response);
    let request = port.request();
    assert_eq!(request.action_ref(), "actions/checkout@v4");
    assert_eq!(request.repo_path(), repository_path);
    assert!(request.env().is_empty());
    let step = JsonStepTextCodec
        .decode(request.step())
        .expect("serialized action step");
    assert_eq!(step.uses(), Some("actions/checkout@v4"));
    assert_eq!(
        step.with(),
        &HashMap::from([(String::from("ref"), String::from("main"))])
    );
    assert_eq!(
        request.context(),
        &EvaluationContext::new().with_root(
            "inputs",
            ContextValue::mapping([(String::from("ref"), ContextValue::text("main"))]),
        )
    );
}
