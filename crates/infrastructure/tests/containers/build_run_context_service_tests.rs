#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use ephact::{
        application::{
            dtos::requests::BuildRunContextRequest, ports::outbound::BuildRunContextPort,
        },
        domain::{
            RepoPath, Repository, RepositoryName, WorkflowRunConfig,
            messages::commands::ExecuteWorkflowPayload,
            value_objects::{ContextValue, EventPath, Secret, WorkflowEvent, WorkflowInput},
        },
        infrastructure::{containers::BuildRunContextService, workflows::GithubContextBuilder},
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
    fn context_with_event_payload(
        payload: &str,
    ) -> ephact::domain::value_objects::EvaluationContext {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".git")).unwrap();
        let event_path = tmp.path().join("event.json");
        fs::write(&event_path, payload).unwrap();
        let config = WorkflowRunConfig::new()
            .with_event(WorkflowEvent::new("pull_request".into()))
            .with_event_path(EventPath::new(event_path).unwrap());
        let repo = repository(tmp.path());
        let command =
            ExecuteWorkflowPayload::new(String::new(), config, repo, "run-1".to_owned(), false);
        GithubContextBuilder::default().build(&command).unwrap()
    }

    #[test]
    fn execute_exposes_raw_event_payload_and_pull_request_condition_fields() {
        let context = context_with_event_payload(
            r#"{"action":"closed","pull_request":{"merged":true,"head":{"ref":"feature"},"base":{"ref":"main"}},"ref":"refs/heads/main","after":"abc123","sender":{"login":"octocat"},"repository":{"full_name":"octocat/example"},"workflow":"CI","job":"build"}"#,
        );

        let github = context.get("github").unwrap();
        assert_eq!(
            github
                .property("event")
                .and_then(|event| event.property("pull_request"))
                .and_then(|pull_request| pull_request.property("merged")),
            Some(&ContextValue::Boolean(true))
        );
        assert_eq!(
            github.property("action"),
            Some(&ContextValue::text("closed"))
        );
        assert_eq!(
            github.property("ref"),
            Some(&ContextValue::text("refs/heads/main"))
        );
        assert_eq!(
            github.property("ref_name"),
            Some(&ContextValue::text("main"))
        );
        assert_eq!(
            github.property("ref_type"),
            Some(&ContextValue::text("branch"))
        );
        assert_eq!(
            github.property("head_ref"),
            Some(&ContextValue::text("feature"))
        );
        assert_eq!(
            github.property("base_ref"),
            Some(&ContextValue::text("main"))
        );
    }

    #[test]
    fn execute_exposes_github_identity_fields_from_event_payload() {
        let context = context_with_event_payload(
            r#"{"action":"closed","pull_request":{"merged":true},"ref":"refs/heads/main","after":"abc123","sender":{"login":"octocat"},"repository":{"full_name":"octocat/example"},"workflow":"CI","job":"build"}"#,
        );

        let github = context.get("github").unwrap();
        assert_eq!(github.property("sha"), Some(&ContextValue::text("abc123")));
        assert_eq!(
            github.property("actor"),
            Some(&ContextValue::text("octocat"))
        );
        assert_eq!(
            github.property("repository"),
            Some(&ContextValue::text("octocat/example"))
        );
        assert_eq!(
            github.property("repository_owner"),
            Some(&ContextValue::text("octocat"))
        );
        assert_eq!(github.property("workflow"), Some(&ContextValue::text("CI")));
        assert_eq!(github.property("job"), Some(&ContextValue::text("build")));
        assert_eq!(
            github.property("run_id"),
            Some(&ContextValue::text("run-1"))
        );
    }

    #[test]
    fn execute_keeps_an_unmerged_pull_request_condition_falsy() {
        let context = context_with_event_payload(
            r#"{"pull_request":{"merged":false},"ref":"refs/heads/main"}"#,
        );

        assert_eq!(
            context
                .get("github")
                .unwrap()
                .property("event")
                .and_then(|event| event.property("pull_request"))
                .and_then(|pull_request| pull_request.property("merged")),
            Some(&ContextValue::Boolean(false))
        );
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
    #[test]
    fn execute_preserves_the_raw_event_payload_for_container_setup() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".git")).unwrap();
        let payload = r#"{"z":1,"action":"opened"}"#;
        let event_path = temp.path().join("event.json");
        fs::write(&event_path, payload).unwrap();
        let config = WorkflowRunConfig::new()
            .with_event(WorkflowEvent::new("pull_request".into()))
            .with_event_path(EventPath::new(event_path).unwrap());
        let command = ExecuteWorkflowPayload::new(
            String::new(),
            config,
            repository(temp.path()),
            "run-1".to_owned(),
            false,
        );

        let context = GithubContextBuilder::default()
            .build_with_payload(&command)
            .unwrap();

        assert_eq!(context.event_payload(), Some(payload));
    }
}
