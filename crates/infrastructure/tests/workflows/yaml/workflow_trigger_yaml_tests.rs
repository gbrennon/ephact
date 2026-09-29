use ephact::{
    domain::value_objects::{TriggerKind, WorkflowTrigger},
    infrastructure::workflows::yaml::WorkflowTriggerYaml,
};

fn triggers_from(yaml: &str) -> Vec<WorkflowTrigger> {
    serde_yaml::from_str::<WorkflowTriggerYaml>(yaml)
        .unwrap()
        .into_domain()
}

#[test]
fn a_scalar_event_becomes_one_domain_trigger() {
    let triggers = triggers_from("push");

    assert_eq!(triggers.len(), 1);
    assert_eq!(triggers[0].kind(), TriggerKind::Push);
}

#[test]
fn a_sequence_becomes_multiple_domain_triggers() {
    let triggers = triggers_from("[push, pull_request]");

    assert_eq!(
        triggers
            .iter()
            .map(WorkflowTrigger::kind)
            .collect::<Vec<_>>(),
        vec![TriggerKind::Push, TriggerKind::PullRequest]
    );
}

#[test]
fn a_manual_event_becomes_a_manual_trigger_with_inputs() {
    let yaml = "workflow_dispatch:\n  inputs:\n    name:\n      description: 'Name to greet'\n      required: true\n      type: string\n";

    let triggers = triggers_from(yaml);

    assert_eq!(triggers[0].kind(), TriggerKind::Manual);
    assert!(triggers[0].inputs().unwrap().contains_key("name"));
}

#[test]
fn a_schedule_event_becomes_a_schedule_trigger() {
    let yaml = "schedule:\n  cron: ['0 0 * * *']\n";

    let triggers = triggers_from(yaml);

    assert_eq!(triggers[0].kind(), TriggerKind::Schedule);
    assert_eq!(triggers[0].expressions(), &["0 0 * * *".to_string()]);
}

#[test]
fn a_mapping_filter_becomes_generic_ref_patterns() {
    let yaml = "push:\n  branches: [main]\n  branches-ignore: [develop]\n  paths: [src/**]\n";

    let triggers = triggers_from(yaml);
    let filter = triggers[0].filter().unwrap();

    assert_eq!(filter.included_refs().len(), 2);
    assert_eq!(filter.excluded_refs().len(), 1);
}
