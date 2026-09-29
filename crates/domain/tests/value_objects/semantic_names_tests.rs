use ephact_domain::{JobName, WorkflowEvent, WorkflowInput, WorkflowPath};

#[test]
fn workflow_types_describe_the_domain_without_tool_names() {
    let event = WorkflowEvent::new("push".into());
    let input = WorkflowInput::new("environment".into(), "staging".into());
    let job = JobName::new("test".into());
    let workflow = WorkflowPath::new(".github/workflows/ci.yml".into());

    assert_eq!(event.as_str(), "push");
    assert_eq!(input.key(), "environment");
    assert_eq!(input.value(), "staging");
    assert_eq!(job.as_str(), "test");
    assert_eq!(workflow.as_str(), ".github/workflows/ci.yml");
}
