use std::{collections::HashMap, path::PathBuf, sync::Arc};

use ephact::{
    application::{
        dtos::{
            requests::ExecuteActionRequest,
            responses::{
                ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
                JobSummaryResponse, WorkflowExecutionResponse,
            },
        },
        errors::ExecuteActionError,
        ports::{
            inbound::{
                execute_action_port::ExecuteActionPort, execute_job_port::ExecuteJobPort,
                execute_step_port::ExecuteStepPort, execute_workflow_port::ExecuteWorkflowPort,
            },
            outbound::{
                ActionCommandPublisherPort, JobCommandPublisherPort, StepCommandPublisherPort,
                StepTextCodecPort, WorkflowCommandPublisherPort, container_port::ContainerPort,
            },
        },
    },
    domain::{
        RepoPath, Repository, RepositoryName, WorkflowRunConfig,
        aggregates::Workflow,
        entities::Job,
        errors::StepError,
        messages::commands::{
            ExecuteActionPayload, ExecuteJobPayload, ExecuteStepPayload, ExecuteWorkflowPayload,
        },
        value_objects::{EvaluationContext, JobName, WorkflowTrigger},
    },
    infrastructure::{
        actions::ActionCommandHandler,
        jobs::JobCommandHandler,
        messaging::{CommandPublisherAdapter, DeferredCommandBus, InMemoryCommandBus},
        steps::{JsonStepTextCodec, StepCommandHandler},
        workflows::{WorkflowCommandHandler, actions::StepYaml},
    },
};
use parking_lot::Mutex;

use crate::common::fakes::stub_container::StubContainer;

fn bound_publisher(
    workflow: WorkflowCommandHandler,
    job: JobCommandHandler,
    step: StepCommandHandler,
    action: ActionCommandHandler,
) -> CommandPublisherAdapter {
    let deferred = Arc::new(DeferredCommandBus::new());
    deferred.bind(InMemoryCommandBus::new(
        Box::new(workflow),
        Box::new(job),
        Box::new(step),
        Box::new(action),
    ));
    CommandPublisherAdapter::new(deferred)
}

fn stub_handlers() -> (
    WorkflowCommandHandler,
    JobCommandHandler,
    StepCommandHandler,
    ActionCommandHandler,
) {
    (
        WorkflowCommandHandler::new(Box::new(StubWorkflowPort)),
        JobCommandHandler::new(Box::new(StubJobPort)),
        StepCommandHandler::new(Box::new(|_| Box::new(StubStepPort))),
        ActionCommandHandler::new(Box::new(|_| Box::new(StubActionPort))),
    )
}

fn stub_container() -> Arc<dyn ContainerPort> {
    Arc::new(StubContainer)
}

struct StubWorkflowPort;
impl ExecuteWorkflowPort for StubWorkflowPort {
    fn execute(
        &self,
        _request: ephact::application::dtos::requests::ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ephact::application::errors::ExecuteWorkflowError> {
        Ok(WorkflowExecutionResponse::new(
            "dispatched-wf".to_string(),
            Vec::new(),
            vec!["c1".to_string()],
            true,
        ))
    }
}

struct RecordingWorkflowPort {
    selected_job: Arc<Mutex<Option<String>>>,
}

impl ExecuteWorkflowPort for RecordingWorkflowPort {
    fn execute(
        &self,
        request: ephact::application::dtos::requests::ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ephact::application::errors::ExecuteWorkflowError> {
        *self.selected_job.lock() = request.selected_job().map(str::to_owned);
        Ok(WorkflowExecutionResponse::new(
            "dispatched-wf".to_string(),
            Vec::new(),
            vec!["c1".to_string()],
            true,
        ))
    }
}

struct StubJobPort;
impl ExecuteJobPort for StubJobPort {
    fn execute(
        &self,
        _request: ephact::application::dtos::requests::ExecuteJobRequest,
        _run: &ephact::domain::entities::JobRun,
        _workflow: &ephact::domain::aggregates::Workflow,
    ) -> Result<JobExecutionResponse, ephact::application::errors::ExecuteJobError> {
        Ok(JobExecutionResponse::new(
            JobSummaryResponse::new(
                "j1".to_string(),
                Some("job 1".to_string()),
                Vec::new(),
                true,
            ),
            "c1".to_string(),
        ))
    }
}

struct StubStepPort;
impl ExecuteStepPort for StubStepPort {
    fn execute(
        &self,
        request: ephact::application::dtos::requests::ExecuteStepRequest,
    ) -> Result<ExecutedStepResponse, ephact::application::errors::ExecuteStepError> {
        Ok(ExecutedStepResponse::new(
            JsonStepTextCodec.decode(request.step()).unwrap(),
            ExecuteActionResponse::new(0, "step out".to_string(), String::new()),
        ))
    }
}

struct StubActionPort;
impl ExecuteActionPort for StubActionPort {
    fn execute(
        &self,
        _request: ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, ephact::application::errors::ExecuteActionError> {
        Ok(ExecuteActionResponse::new(
            0,
            "action out".to_string(),
            String::new(),
        ))
    }
}

struct FailingActionPort;
impl ExecuteActionPort for FailingActionPort {
    fn execute(
        &self,
        _request: ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, ExecuteActionError> {
        Err(ExecuteActionError::Step(
            StepError::new("action failed".to_string())
                .with_stdout("action output".to_string())
                .with_stderr("action error".to_string()),
        ))
    }
}

fn sample_workflow_command() -> ExecuteWorkflowPayload {
    sample_workflow_command_with_config(WorkflowRunConfig::new())
}

fn sample_workflow_command_with_config(config: WorkflowRunConfig) -> ExecuteWorkflowPayload {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join(".git")).unwrap();
    let repository = Repository::new(
        RepoPath::new(tmp.path().to_path_buf()).unwrap(),
        RepositoryName::new("test-repo".into()).unwrap(),
    );
    ExecuteWorkflowPayload::new(
        "name: CI\non: [push]\n".to_string(),
        config,
        repository,
        "test-run".to_string(),
        false,
    )
}

fn checkout_action_payload() -> ExecuteActionPayload {
    let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
        .unwrap()
        .into_domain();
    ExecuteActionPayload::new(
        "actions/checkout@v4".into(),
        step,
        PathBuf::from("/repo"),
        HashMap::new(),
        EvaluationContext::new(),
    )
}

#[test]
fn publisher_routes_workflow_command_to_workflow_handler() {
    let (workflow, job, step, action) = stub_handlers();
    let publisher = bound_publisher(workflow, job, step, action);

    let result =
        WorkflowCommandPublisherPort::publish(&publisher, sample_workflow_command()).unwrap();

    assert_eq!(result.workflow_name(), "dispatched-wf");
}

#[test]
fn publisher_routes_selected_job_to_workflow_handler() {
    let selected_job = Arc::new(Mutex::new(None));
    let workflow = WorkflowCommandHandler::new(Box::new(RecordingWorkflowPort {
        selected_job: selected_job.clone(),
    }));
    let (_, job, step, action) = stub_handlers();
    let publisher = bound_publisher(workflow, job, step, action);
    let config = WorkflowRunConfig::new().with_job(JobName::new("publish".to_string()));

    WorkflowCommandPublisherPort::publish(&publisher, sample_workflow_command_with_config(config))
        .unwrap();

    assert_eq!(selected_job.lock().as_deref(), Some("publish"));
}

#[test]
fn publisher_routes_action_command_to_action_handler() {
    let (workflow, job, step, action) = stub_handlers();
    let publisher = bound_publisher(workflow, job, step, action);

    let result = ActionCommandPublisherPort::publish(
        &publisher,
        checkout_action_payload(),
        stub_container(),
    )
    .unwrap();

    assert_eq!(result.stdout(), "action out");
    assert_eq!(result.exit_code(), 0);
}

#[test]
fn action_handler_preserves_failed_action_output() {
    let handler = ActionCommandHandler::new(Box::new(|_| Box::new(FailingActionPort)));
    let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
        .unwrap()
        .into_domain();
    let command = ExecuteActionPayload::new(
        "actions/checkout@v4".into(),
        step,
        PathBuf::from("/repo"),
        HashMap::new(),
        EvaluationContext::new(),
    );

    let error = handler.handle(command, stub_container()).unwrap_err();

    assert_eq!(error.message(), "action failed");
    assert_eq!(error.stdout(), "action output");
    assert_eq!(error.stderr(), "action error");
}

struct EchoJobPort;

impl ExecuteJobPort for EchoJobPort {
    fn execute(
        &self,
        request: ephact::application::dtos::requests::ExecuteJobRequest,
        run: &ephact::domain::entities::JobRun,
        _workflow: &ephact::domain::aggregates::Workflow,
    ) -> Result<JobExecutionResponse, ephact::application::errors::ExecuteJobError> {
        Ok(JobExecutionResponse::new(
            JobSummaryResponse::new(
                run.job_id().to_string(),
                run.workflow_name().map(str::to_string),
                Vec::new(),
                true,
            ),
            request.repo_path().display().to_string(),
        ))
    }
}

struct EchoStepPort;

impl ExecuteStepPort for EchoStepPort {
    fn execute(
        &self,
        request: ephact::application::dtos::requests::ExecuteStepRequest,
    ) -> Result<ExecutedStepResponse, ephact::application::errors::ExecuteStepError> {
        Ok(ExecutedStepResponse::new(
            JsonStepTextCodec.decode(request.step()).unwrap(),
            ExecuteActionResponse::new(
                0,
                request.env().get("MARKER").cloned().unwrap_or_default(),
                request.repo_path().display().to_string(),
            ),
        ))
    }
}

fn workflow_named(name: &str) -> Workflow {
    Workflow::new(
        Some(name.to_string()),
        vec![WorkflowTrigger::PullRequest(None)],
        HashMap::new(),
        HashMap::new(),
    )
}

#[test]
fn publisher_routes_job_command_to_job_handler_with_command_payload() {
    let publisher = bound_publisher(
        WorkflowCommandHandler::new(Box::new(StubWorkflowPort)),
        JobCommandHandler::new(Box::new(EchoJobPort)),
        StepCommandHandler::new(Box::new(|_| Box::new(StubStepPort))),
        ActionCommandHandler::new(Box::new(|_| Box::new(StubActionPort))),
    );
    let repo_path = PathBuf::from("/repo/job");
    let command = ExecuteJobPayload::new(
        Job::default(),
        "build-job".to_string(),
        workflow_named("Build"),
        repo_path.clone(),
        EvaluationContext::new(),
    )
    .with_run_id("test-run".to_string())
    .with_allow_repo_writes(false);

    let result = JobCommandPublisherPort::publish(&publisher, command).unwrap();
    assert_eq!(result.job_summary().job_id(), "build-job");
    assert_eq!(result.job_summary().name(), Some("Build"));
    assert_eq!(result.container_name(), repo_path.display().to_string());
}

#[test]
fn publisher_routes_step_command_to_step_handler_with_command_payload() {
    let publisher = bound_publisher(
        WorkflowCommandHandler::new(Box::new(StubWorkflowPort)),
        JobCommandHandler::new(Box::new(StubJobPort)),
        StepCommandHandler::new(Box::new(|_| Box::new(EchoStepPort))),
        ActionCommandHandler::new(Box::new(|_| Box::new(StubActionPort))),
    );
    let step = serde_yaml::from_str::<StepYaml>("run: echo hello")
        .unwrap()
        .into_domain();
    let command = ExecuteStepPayload::new(
        step,
        HashMap::from([("MARKER".to_string(), "step-marker".to_string())]),
        EvaluationContext::new(),
        PathBuf::from("/repo/step"),
    );

    let result = StepCommandPublisherPort::publish(&publisher, command, stub_container()).unwrap();
    assert_eq!(result.step().run(), Some("echo hello"));
    assert_eq!(result.response().stdout(), "step-marker");
    assert_eq!(result.response().stderr(), "/repo/step");
}
