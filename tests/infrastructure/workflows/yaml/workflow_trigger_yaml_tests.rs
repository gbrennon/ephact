use ephact::{
    domain::value_objects::WorkflowTrigger, infrastructure::workflows::yaml::WorkflowTriggerYaml,
};

fn trigger_from(yaml: &str) -> WorkflowTrigger {
    serde_yaml::from_str::<WorkflowTriggerYaml>(yaml)
        .unwrap()
        .into_domain()
}

#[test]
fn a_scalar_event_becomes_a_single_trigger() {
    let trigger = trigger_from("push");

    assert!(trigger.is_single("push"));
    assert_eq!(trigger.event_names(), vec!["push"]);
}

#[test]
fn a_sequence_of_events_becomes_a_multiple_trigger() {
    let trigger = trigger_from("[push, pull_request]");

    assert!(trigger.is_multiple());
    assert_eq!(trigger.event_names(), vec!["push", "pull_request"]);
}

#[test]
fn a_mapping_of_events_becomes_a_configured_trigger() {
    let yaml = "push:\n  branches: [main, develop]\npull_request:\n  types: [opened]\n";

    let trigger = trigger_from(yaml);

    assert!(trigger.has_types());
    assert!(trigger.has_event("push"));
    assert!(trigger.has_event("pull_request"));
    assert!(!trigger.has_event("schedule"));
}

#[test]
fn a_mapping_entry_without_filters_still_declares_its_event() {
    let trigger = trigger_from("push:\npull_request:\n");

    let mut names = trigger.event_names();
    names.sort_unstable();
    assert_eq!(names, vec!["pull_request", "push"]);
}

#[test]
fn workflow_dispatch_inputs_survive_the_mapping() {
    let yaml = "workflow_dispatch:\n  inputs:\n    name:\n      description: 'Name to greet'\n      required: true\n      type: string\n";

    let trigger = trigger_from(yaml);

    let inputs = trigger.workflow_dispatch_inputs().unwrap();
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs.get("name").map(|input| input.required()), Some(true));
}

#[test]
fn a_schedule_entry_keeps_its_cron_event() {
    let trigger = trigger_from("schedule:\n  cron: ['0 0 * * *']\n");

    assert!(trigger.has_event("schedule"));
}

#[test]
fn a_non_string_scalar_is_rejected() {
    assert!(serde_yaml::from_str::<WorkflowTriggerYaml>("123").is_err());
}

#[test]
fn a_missing_trigger_defaults_to_push() {
    assert_eq!(
        WorkflowTriggerYaml::default().into_domain(),
        WorkflowTrigger::Single("push".to_owned())
    );
}
