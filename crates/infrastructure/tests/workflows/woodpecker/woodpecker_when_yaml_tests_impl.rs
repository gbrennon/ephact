use ephact::{
    domain::value_objects::WorkflowTrigger,
    infrastructure::workflows::woodpecker::WoodpeckerWhenYaml,
};

fn triggers_from(yaml: &str) -> Vec<WorkflowTrigger> {
    serde_yaml::from_str::<WoodpeckerWhenYaml>(yaml)
        .unwrap()
        .into_triggers()
}

#[test]
fn a_single_event_scalar_maps_to_one_trigger() {
    assert_eq!(triggers_from("event: push"), [WorkflowTrigger::Push(None)]);
}

#[test]
fn an_event_list_maps_to_a_trigger_each() {
    let triggers = triggers_from("event: [push, pull_request]");

    assert!(triggers.contains(&WorkflowTrigger::Push(None)));
    assert!(triggers.contains(&WorkflowTrigger::PullRequest(None)));
}

#[test]
fn unknown_events_are_ignored() {
    assert!(triggers_from("event: deployment").is_empty());
}

#[test]
fn a_missing_event_yields_no_triggers() {
    assert!(triggers_from("{}").is_empty());
}
