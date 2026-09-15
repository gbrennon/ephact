#[cfg(test)]
mod tests {
    use std::path::Path;

    use ephact::application::dtos::requests::RunWorkflowRequest;
    use ephact::application::dtos::responses::RunSummaryResponse;
    use ephact::application::dtos::responses::WorkflowExecutionResponse;
    use ephact::application::ports::inbound::RunWorkflowPort;
    use ephact::application::services::run_workflow_service::RunWorkflowService;
    use ephact::domain::ActRunConfig;
    use ephact::domain::RepoPath;
    use ephact::domain::Repository;
    use ephact::domain::RepositoryName;
    use ephact::domain::messages::events::DomainEvent;

    use crate::common::fakes::{
        fake_command_bus::FakeCommandBus,
        fake_detect_workflow_trigger_port::FakeDetectWorkflowTriggerPort,
        fake_event_bus::FakeEventBus, fake_workflow_source::FakeWorkflowSource,
    };

    fn make_repo(path: &Path) -> Repository {
        let git_dir = path.join(".git");
        if !git_dir.exists() {
            std::fs::create_dir_all(&git_dir).ok();
        }
        let repo_path = RepoPath::new(path.to_path_buf()).unwrap();
        let name = RepositoryName::new("test-repo".to_string()).unwrap();
        Repository::new(repo_path, name)
    }

    fn primitive_request(config: ActRunConfig, repository: Repository) -> RunWorkflowRequest {
        RunWorkflowRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
            config.workflow().map(|value| value.as_str().to_string()),
            config.job().map(|value| value.as_str().to_string()),
            config.event().map(|value| value.as_str().to_string()),
            config
                .inputs()
                .iter()
                .map(|input| (input.key().to_string(), input.value().to_string()))
                .collect(),
            config
                .secrets()
                .iter()
                .map(|secret| (secret.name().to_string(), secret.value().to_string()))
                .collect(),
            config.all_workflows(),
            config.allow_repo_writes(),
            config.allow_real_container(),
            config.allow_real_fetcher(),
            config.allow_network(),
            config.run_id().to_string(),
        )
    }

    #[test]
    fn execute_runs_workflow_and_publishes_lifecycle_events() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());

        let workflow_source =
            FakeWorkflowSource::new().with_workflow_content("name: CI\non: pull_request\njobs: {}");
        let command_bus =
            FakeCommandBus::new().with_workflow_result(WorkflowExecutionResponse::new(
                "CI".to_string(),
                Vec::new(),
                vec!["test-container-1".to_string()],
                true,
            ));
        let event_bus = FakeEventBus::new();
        let config = ActRunConfig::new();
        let run_id = config.run_id().to_string();
        let repository_path = temp.path().display().to_string();

        let service = RunWorkflowService::new(
            Box::new(workflow_source),
            Box::new(command_bus.clone()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
        );
        let request = primitive_request(config, repo);

        let summary: RunSummaryResponse = service.execute(request).unwrap();

        assert_eq!(summary.name(), "CI");
        assert!(summary.success());
        assert_eq!(command_bus.dispatched_workflows.lock().len(), 1);

        let events = event_bus.events();
        assert_eq!(events.len(), 2);
        let DomainEvent::RunStarted(payload) = &events[0] else {
            panic!("expected RunStarted event");
        };
        assert_eq!(payload.run_id(), run_id);
        assert_eq!(payload.repository_path(), repository_path);
        let DomainEvent::ActRunCompleted(payload) = &events[1] else {
            panic!("expected ActRunCompleted event");
        };
        assert_eq!(payload.run_id(), run_id);
        assert_eq!(payload.repository_path(), repository_path);
        assert!(payload.success());
        assert_eq!(
            payload.container_names(),
            vec!["test-container-1".to_string()]
        );
    }

    #[test]
    fn execute_publishes_run_failed_when_workflow_read_fails() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let source = FakeWorkflowSource::new().failing_read_workflow("cannot read workflow");
        let event_bus = FakeEventBus::new();
        let config = ActRunConfig::new();
        let run_id = config.run_id().to_string();

        let service = RunWorkflowService::new(
            Box::new(source),
            Box::new(FakeCommandBus::new()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
        );

        let error = service
            .execute(primitive_request(config, repo))
            .unwrap_err();

        assert_eq!(error.to_string(), "cannot read workflow");
        let events = event_bus.events();
        assert_eq!(events.len(), 2);
        let DomainEvent::RunFailed(payload) = &events[1] else {
            panic!("expected RunFailed event");
        };
        assert_eq!(payload.run_id(), run_id);
        assert_eq!(payload.error(), "cannot read workflow");
    }

    #[test]
    fn execute_publishes_run_failed_when_trigger_is_missing() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let workflow_source =
            FakeWorkflowSource::new().with_workflow_content("name: CI\non: merge_group\njobs: {}");
        let event_bus = FakeEventBus::new();
        let config = ActRunConfig::new();
        let run_id = config.run_id().to_string();
        let service = RunWorkflowService::new(
            Box::new(workflow_source),
            Box::new(FakeCommandBus::new()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::never_triggering()),
        );

        let error = service
            .execute(primitive_request(config, repo))
            .unwrap_err();

        assert!(error.to_string().contains("pull_request"));
        let events = event_bus.events();
        assert_eq!(events.len(), 2);
        let DomainEvent::RunFailed(payload) = &events[1] else {
            panic!("expected RunFailed event");
        };
        assert_eq!(payload.run_id(), run_id);
        assert_eq!(
            payload.error(),
            "workflow does not define a pull_request event"
        );
    }

    #[test]
    fn execute_rejects_workflows_without_pull_request_event() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let workflow_source =
            FakeWorkflowSource::new().with_workflow_content("name: CI\non: merge_group\njobs: {}");
        let command_bus = FakeCommandBus::new();
        let event_bus = FakeEventBus::new();
        let service = RunWorkflowService::new(
            Box::new(workflow_source),
            Box::new(command_bus.clone()),
            Box::new(event_bus),
            Box::new(FakeDetectWorkflowTriggerPort::never_triggering()),
        );
        let request = primitive_request(ActRunConfig::new(), repo);

        let error = service.execute(request).unwrap_err();

        assert!(error.to_string().contains("pull_request"));
        assert!(command_bus.dispatched_workflows.lock().is_empty());
    }
}
