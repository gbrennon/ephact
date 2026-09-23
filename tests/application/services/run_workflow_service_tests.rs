#[cfg(test)]
mod tests {
    use std::path::Path;

    use ephact::{
        application::{
            dtos::{
                requests::RunWorkflowRequest,
                responses::{RunSummaryResponse, WorkflowExecutionResponse},
            },
            ports::inbound::RunWorkflowPort,
            services::run_workflow_service::RunWorkflowService,
        },
        domain::{
            ActRunConfig, RepoPath, Repository, RepositoryName, messages::events::DomainEvent,
            value_objects::ActEvent,
        },
    };

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
        RunWorkflowRequest::from_domain(&repository, &config)
    }

    #[tokio::test]
    async fn execute_runs_workflow_and_publishes_lifecycle_events() {
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
        let config = ActRunConfig::new().with_event(ActEvent::new("pull_request".to_owned()));
        let run_id = config.run_id().to_string();
        let repository_path = temp.path().display().to_string();

        let service = RunWorkflowService::new(
            Box::new(workflow_source),
            Box::new(command_bus.clone()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
        );
        let request = primitive_request(config, repo);

        let summary: RunSummaryResponse = service.execute(request).await.unwrap();

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

    #[tokio::test]
    async fn execute_publishes_run_failed_when_workflow_read_fails() {
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
            .await
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

    #[tokio::test]
    async fn execute_publishes_run_failed_when_event_is_missing() {
        let temp = tempfile::tempdir().unwrap();
        let repo = make_repo(temp.path());
        let workflow_source =
            FakeWorkflowSource::new().with_workflow_content("name: CI\non: merge_group\njobs: {}");
        let event_bus = FakeEventBus::new();
        let command_bus = FakeCommandBus::new();
        let config = ActRunConfig::new();
        let run_id = config.run_id().to_string();
        let service = RunWorkflowService::new(
            Box::new(workflow_source),
            Box::new(command_bus.clone()),
            Box::new(event_bus.clone()),
            Box::new(FakeDetectWorkflowTriggerPort::never_triggering()),
        );

        let error = service
            .execute(primitive_request(config, repo))
            .await
            .unwrap_err();

        assert_eq!(error.to_string(), "workflow event must be specified");
        let events = event_bus.events();
        assert_eq!(events.len(), 2);
        let DomainEvent::RunFailed(payload) = &events[1] else {
            panic!("expected RunFailed event");
        };
        assert_eq!(payload.run_id(), run_id);
        assert_eq!(payload.error(), "workflow event must be specified");
        assert!(command_bus.dispatched_workflows.lock().is_empty());
    }

    #[tokio::test]
    async fn execute_rejects_workflows_without_an_explicit_event() {
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

        let error = service.execute(request).await.unwrap_err();

        assert_eq!(error.to_string(), "workflow event must be specified");
        assert!(command_bus.dispatched_workflows.lock().is_empty());
    }
}
