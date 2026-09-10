use ephact::{domain::aggregates::Workflow, infrastructure::workflows::yaml::WorkflowYaml};

fn workflow_from(yaml: &str) -> Workflow {
    serde_yaml::from_str::<WorkflowYaml>(yaml)
        .unwrap()
        .into_domain()
}

#[test]
fn a_minimal_workflow_keeps_its_jobs() {
    let yaml = "on: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo hello\n";

    let workflow = workflow_from(yaml);

    assert!(workflow.name().is_none());
    assert_eq!(workflow.jobs().len(), 1);
    assert!(workflow.jobs().contains_key("build"));
}

#[test]
fn the_on_entry_becomes_the_workflow_trigger() {
    let workflow = workflow_from("on: [push, pull_request]\njobs: {}\n");

    assert!(workflow.trigger().has_event("push"));
    assert!(workflow.trigger().has_event("pull_request"));
}

#[test]
fn a_workflow_without_an_on_entry_defaults_to_push() {
    let workflow = workflow_from("jobs: {}\n");

    assert!(workflow.trigger().has_event("push"));
}

#[test]
fn the_name_and_environment_reach_the_domain_workflow() {
    let yaml = "name: CI\non: push\nenv:\n  RUST_BACKTRACE: \"1\"\njobs: {}\n";

    let workflow = workflow_from(yaml);

    assert_eq!(workflow.name(), Some("CI"));
    assert_eq!(
        workflow.env().get("RUST_BACKTRACE").map(String::as_str),
        Some("1")
    );
}

#[test]
fn run_defaults_reach_the_domain_workflow() {
    let yaml =
        "on: push\ndefaults:\n  run:\n    shell: bash\n    working-directory: ./src\njobs: {}\n";

    let workflow = workflow_from(yaml);

    let run_defaults = workflow.defaults().unwrap().run().unwrap().clone();
    assert_eq!(run_defaults.shell(), Some("bash"));
    assert_eq!(run_defaults.working_directory(), Some("./src"));
}

#[test]
fn permissions_and_concurrency_reach_the_domain_workflow() {
    let yaml = "on: push\npermissions:\n  pull-requests: write\nconcurrency:\n  group: ci\n  cancel-in-progress: true\njobs: {}\n";

    let workflow = workflow_from(yaml);

    assert_eq!(
        workflow.permissions().unwrap().pull_requests(),
        Some("write")
    );
    let concurrency = workflow.concurrency().unwrap();
    assert_eq!(concurrency.group(), "ci");
    assert_eq!(concurrency.cancel_in_progress(), Some(true));
}
