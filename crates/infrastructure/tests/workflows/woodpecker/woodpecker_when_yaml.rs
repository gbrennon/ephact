use ephact::{
    domain::value_objects::{RefPattern, TriggerFilter, WorkflowTrigger},
    infrastructure::workflows::woodpecker::WoodpeckerWhenYaml,
};

struct WoodpeckerWhenYamlTest;

impl WoodpeckerWhenYamlTest {
    fn triggers_from(yaml: &str) -> Vec<WorkflowTrigger> {
        WoodpeckerWhenYaml::parse(yaml).unwrap().into_triggers()
    }
}

#[test]
fn a_single_event_scalar_maps_to_one_trigger() {
    assert_eq!(
        WoodpeckerWhenYamlTest::triggers_from("event: push"),
        [WorkflowTrigger::Push(None)]
    );
}

#[test]
fn an_event_list_maps_to_a_trigger_each() {
    let triggers = WoodpeckerWhenYamlTest::triggers_from("event: [push, pull_request]");

    assert!(triggers.contains(&WorkflowTrigger::Push(None)));
    assert!(triggers.contains(&WorkflowTrigger::PullRequest(None)));
}

#[test]
fn unknown_events_are_ignored() {
    assert!(WoodpeckerWhenYamlTest::triggers_from("event: deployment").is_empty());
}

#[test]
fn a_missing_event_yields_no_triggers() {
    assert!(WoodpeckerWhenYamlTest::triggers_from("{}").is_empty());
}

#[test]
fn a_tag_event_maps_to_a_tag_trigger() {
    assert_eq!(
        WoodpeckerWhenYamlTest::triggers_from("event: tag"),
        [WorkflowTrigger::Tag(None)]
    );
}

#[test]
fn a_tag_event_preserves_its_ref_pattern() {
    let expected = TriggerFilter::new().with_included_ref(RefPattern::tag("refs/tags/v*"));

    assert_eq!(
        WoodpeckerWhenYamlTest::triggers_from("event: tag\nref: refs/tags/v*"),
        [WorkflowTrigger::Tag(Some(expected))]
    );
}
