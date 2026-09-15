#[cfg(test)]
mod tests {
    use std::path::Path;

    use ephact::application::dtos::requests::RunAllWorkflowsRequest;
    use ephact::application::dtos::responses::WorkflowExecutionResponse;
    use ephact::application::ports::inbound::RunAllWorkflowsPort;
    use ephact::application::services::run_all_workflows_service::ALL_WORKFLOWS_SUMMARY_NAME;
    use ephact::application::services::run_all_workflows_service::RunAllWorkflowsService;
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

    fn primitive_request(config: ActRunConfig, repository: Repository) -> RunAllWorkflowsRequest {
        RunAllWorkflowsRequest::new(
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
    fn execute_runs_all_workflows_and_merges_summary() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let workflow_source = FakeWorkflowSource::new().with_all_workflow_contents(vec![
            "name: A\non: pull_request\njobs: {}".into(),
            "name: B\non: pull_request\njobs: {}".into(),
        ]);
        let command_bus =
            FakeCommandBus::new().with_workflow_result(WorkflowExecutionResponse::new(
                "TestWF".to_string(),
                Vec::new(),
                vec!["c-all".to_string()],
                true,
            ));
        let event_bus = FakeEventBus::new();
        let config = ActRunConfig::new();
        let run_id = config.run_id().to_string();
        let repository_path = temp.path().display().to_string();
        let service = RunAllWorkflowsService::new(
            Box::new(workflow_source),
            Box::new(command_bus.clone()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
        );

        let summary = service.execute(primitive_request(config, repo)).unwrap();

        assert!(summary.success());
        assert_eq!(summary.name(), ALL_WORKFLOWS_SUMMARY_NAME);
        assert_eq!(command_bus.dispatched_workflows.lock().len(), 2);
        assert_pull_request_workflows(&command_bus);
        assert_completed_events(&event_bus, &run_id, &repository_path);
    }

    fn assert_pull_request_workflows(command_bus: &FakeCommandBus) {
        let dispatched = command_bus.dispatched_workflows.lock();
        assert!(dispatched.iter().all(|command| {
            command.config().event().map(|event| event.as_str()) == Some("pull_request")
        }));
    }

    fn assert_completed_events(event_bus: &FakeEventBus, run_id: &str, repository_path: &str) {
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
            vec!["c-all".to_string(), "c-all".to_string()]
        );
    }

    #[test]
    fn execute_publishes_run_failed_when_workflow_collection_fails() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let source = FakeWorkflowSource::new().failing_read_all_workflows("cannot list workflows");
        let event_bus = FakeEventBus::new();
        let config = ActRunConfig::new();
        let run_id = config.run_id().to_string();
        let service = RunAllWorkflowsService::new(
            Box::new(source),
            Box::new(FakeCommandBus::new()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
        );

        let error = service
            .execute(primitive_request(config, repo))
            .unwrap_err();

        assert_eq!(error.to_string(), "cannot list workflows");
        let events = event_bus.events();
        assert_eq!(events.len(), 2);
        let DomainEvent::RunFailed(payload) = &events[1] else {
            panic!("expected RunFailed event");
        };
        assert_eq!(payload.run_id(), run_id);
        assert_eq!(payload.error(), "cannot list workflows");
    }

    #[test]
    fn execute_skips_non_pull_request_workflows() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let workflow_source = FakeWorkflowSource::new().with_all_workflow_contents(vec![
            "name: Push\non: push\njobs: {}".into(),
            "name: Merge\non: merge_group\njobs: {}".into(),
            "name: PR\non: pull_request\njobs: {}".into(),
        ]);
        let command_bus = FakeCommandBus::new();
        let event_bus = FakeEventBus::new();
        let service = RunAllWorkflowsService::new(
            Box::new(workflow_source),
            Box::new(command_bus.clone()),
            Box::new(event_bus),
            Box::new(FakeDetectWorkflowTriggerPort::only_for_content_containing(
                "name: PR",
            )),
        );
        let request = primitive_request(ActRunConfig::new(), repo);

        let summary = service.execute(request).unwrap();

        assert!(summary.success());
        let dispatched = command_bus.dispatched_workflows.lock();
        assert_eq!(dispatched.len(), 1);
        assert!(dispatched[0].workflow_content().contains("name: PR"));
        assert_eq!(
            dispatched[0].config().event().map(|event| event.as_str()),
            Some("pull_request")
        );
    }
}
