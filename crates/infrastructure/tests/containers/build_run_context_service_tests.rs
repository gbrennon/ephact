use std::path::Path;

use ephact::{
    application::{dtos::requests::BuildRunContextRequest, ports::outbound::BuildRunContextPort},
    domain::{
        RepoPath, Repository, RepositoryName, WorkflowRunConfig,
        value_objects::{ContextValue, Secret, WorkflowEvent, WorkflowInput},
    },
    infrastructure::containers::BuildRunContextService,
};

fn repository(path: &Path) -> Repository {
    Repository::new(
        RepoPath::new(path.to_path_buf()).unwrap(),
        RepositoryName::new("test-repo".into()).unwrap(),
    )
}

fn context(config: WorkflowRunConfig) -> ephact::domain::value_objects::EvaluationContext {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join(".git")).unwrap();
    let repo = repository(tmp.path());
    BuildRunContextService::new()
        .build(BuildRunContextRequest::new(config, repo))
        .context()
        .clone()
}

#[test]
fn execute_exposes_configured_secrets_under_the_secrets_context() {
    let config =
        WorkflowRunConfig::new().add_secret(Secret::new("TOKEN".into(), "secret-value".into()));

    let context = context(config);

    assert_eq!(
        context.get("secrets").unwrap().property("TOKEN"),
        Some(&ContextValue::text("secret-value"))
    );
}

#[test]
fn execute_exposes_inputs_under_both_inputs_and_the_github_event() {
    let config =
        WorkflowRunConfig::new().add_input(WorkflowInput::new("mode".into(), "staging".into()));

    let context = context(config);

    assert_eq!(
        context.get("inputs").unwrap().property("mode"),
        Some(&ContextValue::text("staging"))
    );
    assert_eq!(
        context
            .get("github")
            .unwrap()
            .property("event")
            .and_then(|value| value.property("inputs"))
            .and_then(|value| value.property("mode")),
        Some(&ContextValue::text("staging"))
    );
}

#[test]
fn execute_defaults_the_event_name_to_workflow_dispatch() {
    let context = context(WorkflowRunConfig::new());

    assert_eq!(
        context.get("github").unwrap().property("event_name"),
        Some(&ContextValue::text("workflow_dispatch"))
    );
}

#[test]
fn execute_honours_the_configured_event_name() {
    let config = WorkflowRunConfig::new().with_event(WorkflowEvent::new("pull_request".into()));

    let context = context(config);

    assert_eq!(
        context.get("github").unwrap().property("event_name"),
        Some(&ContextValue::text("pull_request"))
    );
}

#[test]
fn execute_reports_the_repository_name_and_mounted_workspace() {
    let context = context(WorkflowRunConfig::new());

    assert_eq!(
        context.get("github").unwrap().property("repository"),
        Some(&ContextValue::text("test-repo"))
    );
    assert_eq!(
        context.get("github").unwrap().property("workspace"),
        Some(&ContextValue::text("/workspace"))
    );
}

#[test]
fn execute_reports_the_runner_platform() {
    let context = context(WorkflowRunConfig::new());

    assert_eq!(
        context.get("runner").unwrap().property("os"),
        Some(&ContextValue::text("Linux"))
    );
    assert_eq!(
        context.get("runner").unwrap().property("arch"),
        Some(&ContextValue::text("X64"))
    );
    assert_eq!(
        context.get("runner").unwrap().property("temp"),
        Some(&ContextValue::text("/tmp"))
    );
}
