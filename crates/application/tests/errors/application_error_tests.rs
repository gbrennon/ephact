use std::error::Error;

use ephact_application::errors::ApplicationError;

#[test]
fn workflow_error_preserves_message_and_error_contract() {
    let error = ApplicationError::Workflow("workflow failed".to_owned());

    assert_eq!(error.to_string(), "workflow failed");
    assert_eq!(error.source().map(ToString::to_string), None);
}
