use std::path::Path;

use ephact::{
    application::{
        dtos::{requests::ExecuteWorkflowRequest, responses::WorkflowExecutionResponse},
        ports::inbound::execute_workflow_port::ExecuteWorkflowPort,
    },
    domain::{messages::commands::Event, value_objects::EvaluationContext},
    infrastructure::workflows::execution::execute_workflow_service::ExecuteWorkflowService,
};

use crate::common::fakes::{
    fake_command_bus::FakeCommandBus, fake_event_bus::FakeEventBus,
    fake_workflow_loader_port::FakeWorkflowLoaderPort,
};

const TWO_JOBS: &str = "name: Ci\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n  publish:\n    needs: build\n    runs-on: ubuntu-latest\n    steps:\n      - run: publish\n";

const REQUESTED_CONTENT: &str = "name: Ci\non: push\njobs: {}\n";

fn execute(
    loader: FakeWorkflowLoaderPort,
    command_bus: FakeCommandBus,
) -> Result<WorkflowExecutionResponse, ephact::application::errors::ExecuteWorkflowError> {
    execute_with_file_name(loader, command_bus, None)
}

fn execute_with_file_name(
    loader: FakeWorkflowLoaderPort,
    command_bus: FakeCommandBus,
    file_name: Option<&str>,
) -> Result<WorkflowExecutionResponse, ephact::application::errors::ExecuteWorkflowError> {
    ExecuteWorkflowService::new(
        Box::new(loader),
        Box::new(command_bus),
        Box::new(FakeEventBus::new()),
    )
    .execute(
        ExecuteWorkflowRequest::new(
            REQUESTED_CONTENT.to_string(),
            Path::new("/repo").to_path_buf(),
            EvaluationContext::new(),
            "test-run".to_string(),
            false,
        )
        .with_file_name_opt(file_name),
    )
}

#[test]
fn execute_publishes_a_job_command_per_job_in_dependency_order() {
    let command_bus = FakeCommandBus::new();

    let execution = execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        command_bus.clone(),
    )
    .unwrap();

    assert_eq!(command_bus.dispatched_job_ids(), vec!["build", "publish"]);
    assert_eq!(execution.job_summaries().len(), 2);
}

#[test]
fn execute_skips_a_job_when_one_of_its_dependencies_fails() {
    let execution = execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        FakeCommandBus::new().failing_jobs(vec!["build".to_string()]),
    )
    .unwrap();

    assert!(!execution.success());
    let publish = execution
        .job_summaries()
        .iter()
        .find(|job| job.job_id() == "publish")
        .expect("publish summary");
    assert!(publish.is_skipped());
    assert_eq!(publish.skip_reason(), Some("dependency failed"));
    assert!(publish.steps()[0].is_skipped());
    assert_eq!(publish.steps()[0].skip_reason(), Some("dependency failed"));
}

#[test]
fn execute_runs_independent_jobs_when_another_job_fails() {
    let workflow = "name: Ci\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n  lint:\n    runs-on: ubuntu-latest\n    steps:\n      - run: lint\n  publish:\n    needs: build\n    runs-on: ubuntu-latest\n    steps:\n      - run: publish\n";
    let command_bus = FakeCommandBus::new().failing_jobs(vec!["build".to_string()]);

    let execution = execute(
        FakeWorkflowLoaderPort::holding(workflow),
        command_bus.clone(),
    )
    .unwrap();

    let dispatched = command_bus.dispatched_job_ids();
    assert!(dispatched.contains(&"build".to_string()));
    assert!(dispatched.contains(&"lint".to_string()));
    assert!(!dispatched.contains(&"publish".to_string()));
    let publish = execution
        .job_summaries()
        .iter()
        .find(|job| job.job_id() == "publish")
        .expect("publish summary");
    assert!(publish.is_skipped());
}

#[test]
fn execute_publishes_job_commands_carrying_the_loaded_workflow_and_repo_path() {
    let command_bus = FakeCommandBus::new();

    execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        command_bus.clone(),
    )
    .unwrap();

    let dispatched = command_bus.dispatched_jobs.lock();
    let first = dispatched.first().expect("a job command");
    assert_eq!(first.workflow().name(), Some("Ci"));
    assert_eq!(first.repo_path(), Path::new("/repo"));
}

#[test]
fn execute_reports_the_workflow_name() {
    let execution = execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        FakeCommandBus::new(),
    )
    .unwrap();

    assert_eq!(execution.workflow_name(), "Ci");
}

#[test]
fn execute_reports_the_source_filename_in_progress_events_and_summary() {
    let event_bus = FakeEventBus::new();
    let execution = ExecuteWorkflowService::new(
        Box::new(FakeWorkflowLoaderPort::holding(
            "on: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n",
        )),
        Box::new(FakeCommandBus::new()),
        Box::new(event_bus.clone()),
    )
    .execute(
        ExecuteWorkflowRequest::new(
            REQUESTED_CONTENT.to_string(),
            Path::new("/repo").to_path_buf(),
            EvaluationContext::new(),
            "test-run".to_string(),
            false,
        )
        .with_file_name("ci.yml"),
    )
    .unwrap();

    assert_eq!(execution.workflow_name(), "ci.yml");
    let events = event_bus.events();
    let workflow_started = events.iter().find_map(|event| match event {
        Event::WorkflowStarted(payload) => Some(payload),
        _ => None,
    });
    assert_eq!(workflow_started.unwrap().workflow_name(), "ci.yml");
    let job_started = events.iter().find_map(|event| match event {
        Event::JobStarted(payload) => Some(payload),
        _ => None,
    });
    assert_eq!(job_started.unwrap().workflow_name(), "ci.yml");
    let job_finished = events.iter().find_map(|event| match event {
        Event::JobFinished(payload) => Some(payload),
        _ => None,
    });
    assert_eq!(job_finished.unwrap().workflow_name(), "ci.yml");
}

#[test]
fn execute_names_a_content_only_workflow_after_loading_from_its_file() {
    let execution = execute_with_file_name(
        FakeWorkflowLoaderPort::holding(
            "on: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n",
        ),
        FakeCommandBus::new(),
        Some("ci.yml"),
    )
    .unwrap();

    assert_eq!(execution.workflow_name(), "ci.yml");
}

#[test]
fn execute_uses_the_filename_when_the_loaded_workflow_name_is_blank() {
    let execution = execute_with_file_name(
        FakeWorkflowLoaderPort::holding(
            "name: \"  \"\non: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n",
        ),
        FakeCommandBus::new(),
        Some("ci.yml"),
    )
    .unwrap();

    assert_eq!(execution.workflow_name(), "ci.yml");
}

#[test]
fn execute_names_an_unnamed_workflow_unnamed() {
    let execution = execute(
        FakeWorkflowLoaderPort::holding(
            "on: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: build\n",
        ),
        FakeCommandBus::new(),
    )
    .unwrap();

    assert_eq!(execution.workflow_name(), "unnamed");
}

#[test]
fn execute_returns_every_jobs_container_name() {
    let execution = execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        FakeCommandBus::new(),
    )
    .unwrap();

    assert_eq!(
        execution.container_names(),
        vec![
            "container-build".to_string(),
            "container-publish".to_string()
        ]
    );
}

#[test]
fn execute_fails_the_workflow_when_one_job_fails() {
    let execution = execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        FakeCommandBus::new().failing_jobs(vec!["publish".to_string()]),
    )
    .unwrap();

    assert!(!execution.success());
}

#[test]
fn execute_propagates_a_failed_job_dispatch() {
    let result = execute(
        FakeWorkflowLoaderPort::holding(TWO_JOBS),
        FakeCommandBus::new().failing_job_dispatch("job bus is down"),
    );

    let Err(error) = result else {
        panic!("a failing job dispatch should fail the workflow");
    };
    assert_eq!(error.to_string(), "job bus is down");
}

#[test]
fn execute_errors_on_a_cyclic_dependency() {
    let cyclic = "name: Ci\non: push\njobs:\n  a:\n    needs: b\n    runs-on: ubuntu-latest\n    steps:\n      - run: a\n  b:\n    needs: a\n    runs-on: ubuntu-latest\n    steps:\n      - run: b\n";

    let result = execute(
        FakeWorkflowLoaderPort::holding(cyclic),
        FakeCommandBus::new(),
    );

    assert!(result.is_err());
}

#[test]
fn execute_propagates_a_loader_error() {
    let Err(error) = execute(
        FakeWorkflowLoaderPort::failing("cannot read ci.yml"),
        FakeCommandBus::new(),
    ) else {
        panic!("a failing loader should fail the workflow");
    };

    assert_eq!(error.to_string(), "cannot read ci.yml");
}
