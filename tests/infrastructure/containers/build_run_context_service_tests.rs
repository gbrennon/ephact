#[cfg(test)]
mod tests {
    use ephact::infrastructure::containers::{
        build_run_context_port::BuildRunContextPort,
        build_run_context_service::BuildRunContextService,
    };
    use std::path::Path;

    use ephact::application::dtos::requests::BuildRunContextRequest;
    use ephact::domain::ActRunConfig;
    use ephact::domain::RepoPath;
    use ephact::domain::Repository;
    use ephact::domain::RepositoryName;
    use ephact::domain::value_objects::ActEvent;
    use ephact::domain::value_objects::ActInput;
    use ephact::domain::value_objects::ContextValue;
    use ephact::domain::value_objects::Secret;

    fn repository(path: &Path) -> Repository {
        Repository::new(
            RepoPath::new(path.to_path_buf()).unwrap(),
            RepositoryName::new("test-repo".into()).unwrap(),
        )
    }

    fn context(config: ActRunConfig) -> ephact::domain::value_objects::EvaluationContext {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".git")).unwrap();
        let repo = repository(tmp.path());
        BuildRunContextService::new()
            .execute(BuildRunContextRequest::new(&config, &repo))
            .context()
            .clone()
    }

    #[test]
    fn execute_exposes_configured_secrets_under_the_secrets_context() {
        let config =
            ActRunConfig::new().add_secret(Secret::new("TOKEN".into(), "secret-value".into()));

        let context = context(config);

        assert_eq!(
            context.secrets().property("TOKEN"),
            Some(&ContextValue::text("secret-value"))
        );
    }

    #[test]
    fn execute_exposes_inputs_under_both_inputs_and_the_github_event() {
        let config = ActRunConfig::new().add_input(ActInput::new("mode".into(), "staging".into()));

        let context = context(config);

        assert_eq!(
            context.inputs().property("mode"),
            Some(&ContextValue::text("staging"))
        );
        assert_eq!(
            context
                .github()
                .property("event")
                .and_then(|value| value.property("inputs"))
                .and_then(|value| value.property("mode")),
            Some(&ContextValue::text("staging"))
        );
    }

    #[test]
    fn execute_defaults_the_event_name_to_workflow_dispatch() {
        let context = context(ActRunConfig::new());

        assert_eq!(
            context.github().property("event_name"),
            Some(&ContextValue::text("workflow_dispatch"))
        );
    }

    #[test]
    fn execute_honours_the_configured_event_name() {
        let config = ActRunConfig::new().with_event(ActEvent::new("pull_request".into()));

        let context = context(config);

        assert_eq!(
            context.github().property("event_name"),
            Some(&ContextValue::text("pull_request"))
        );
    }

    #[test]
    fn execute_reports_the_repository_name_and_mounted_workspace() {
        let context = context(ActRunConfig::new());

        assert_eq!(
            context.github().property("repository"),
            Some(&ContextValue::text("test-repo"))
        );
        assert_eq!(
            context.github().property("workspace"),
            Some(&ContextValue::text("/workspace"))
        );
    }

    #[test]
    fn execute_reports_the_runner_platform() {
        let context = context(ActRunConfig::new());

        assert_eq!(
            context.runner().property("os"),
            Some(&ContextValue::text("Linux"))
        );
        assert_eq!(
            context.runner().property("arch"),
            Some(&ContextValue::text("X64"))
        );
        assert_eq!(
            context.runner().property("temp"),
            Some(&ContextValue::text("/tmp"))
        );
    }
}
