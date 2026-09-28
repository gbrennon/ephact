use std::path::Path;

use super::RunRequestBuilder;

#[test]
fn builder_creates_a_request_from_incremental_values() {
    let request = RunRequestBuilder::new()
        .repository_path(Path::new("/repo"))
        .repository_name("repo")
        .workflow(Some("workflow.yml"))
        .inputs([(String::from("name"), String::from("value"))])
        .all_workflows(true)
        .allow_network(true)
        .run_id("run-1")
        .build()
        .expect("complete builder should succeed");

    assert_eq!(request.repository_path(), Path::new("/repo"));
    assert_eq!(request.repository_name(), "repo");
    assert_eq!(request.workflow(), Some("workflow.yml"));
    assert_eq!(
        request.inputs(),
        &[(String::from("name"), String::from("value"))]
    );
    assert!(request.all_workflows());
    assert!(request.allow_network());
    assert_eq!(request.run_id(), "run-1");
}

#[test]
fn builder_rejects_missing_required_values() {
    let error = RunRequestBuilder::new()
        .build()
        .expect_err("missing values should fail");
    assert_eq!(error, "repository path");
}
