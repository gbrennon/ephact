use super::*;

#[test]
fn note_succeeds_and_carries_the_message() {
    let response = ExecuteActionResponse::note("workspace already mounted\n");

    assert_eq!(response.exit_code(), 0);
    assert_eq!(response.stdout(), "workspace already mounted\n");
    assert!(response.stderr().is_empty());
}

#[test]
fn builder_constructs_all_response_fields() {
    let response = ExecuteActionResponse::builder()
        .exit_code(2)
        .stdout("output")
        .stderr("error")
        .build();

    assert_eq!(response.into_parts(), (2, "output".into(), "error".into()));
}
