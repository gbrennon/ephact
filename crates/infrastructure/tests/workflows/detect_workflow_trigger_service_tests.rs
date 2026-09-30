use ephact::{
    application::ports::outbound::DetectWorkflowTriggerPort,
    infrastructure::workflows::DetectWorkflowTriggerService,
};

#[test]
fn detects_pull_request_event_in_woodpecker_pipeline() {
    let content = "name: Demo\nwhen:\n  - event: pull_request\nsteps: []\n";

    let triggers = DetectWorkflowTriggerService::new();

    let triggers_pull_request = triggers.triggers_on_event(content, "pull_request");

    assert!(triggers_pull_request);
}
