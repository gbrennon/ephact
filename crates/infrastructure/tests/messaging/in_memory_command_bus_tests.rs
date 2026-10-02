use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

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
                Command, CommandError, CommandHandlerPort, CommandPublisherPort, CommandResponse,
                StepTextCodecPort, container_port::ContainerPort,
            },
        },
    },
    domain::{
        RepoPath, Repository, RepositoryName, WorkflowRunConfig,
        aggregates::Workflow,
        entities::Job,
        errors::StepError,
        messages::commands::{
            ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
        },
        value_objects::{EvaluationContext, WorkflowTrigger},
    },
    infrastructure::{
        actions::ActionCommandHandler,
        jobs::JobCommandHandler,
        messaging::{
            CommandHandlerAdapter, CommandPublisherAdapter, DeferredCommandBus, InMemoryCommandBus,
        },
        steps::{JsonStepTextCodec, StepCommandHandler},
        workflows::{WorkflowCommandHandler, actions::StepYaml},
    },
};

use crate::common::fakes::stub_container::StubContainer;

fn bound_publisher(
    workflow: WorkflowCommandHandler,
    job: JobCommandHandler,
    step: StepCommandHandler,
    action: ActionCommandHandler,
) -> CommandPublisherAdapter {
    let deferred = Arc::new(DeferredCommandBus::new());
    deferred.bind(InMemoryCommandBus::new(Box::new(
        CommandHandlerAdapter::new(workflow, job, step, action),
    )));
    CommandPublisherAdapter::new(deferred)
}

fn publisher_with_handler(handler: Box<dyn CommandHandlerPort>) -> CommandPublisherAdapter {
    let deferred = Arc::new(DeferredCommandBus::new());
    deferred.bind(InMemoryCommandBus::new(handler));
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

/// Records action commands while still handling workflow commands, proving
/// that centralized routing selects the matching handler.
#[derive(Clone, Default)]
struct SpyCommandHandler {
    action_calls: Arc<AtomicUsize>,
}

impl CommandHandlerPort for SpyCommandHandler {
    fn handle(&self, command: Command) -> Result<CommandResponse, CommandError> {
        match command {
            Command::Workflow(_) => Ok(CommandResponse::Workflow(WorkflowExecutionResponse::new(
                "dispatched-wf".to_string(),
                Vec::new(),
                vec!["c1".to_string()],
                true,
            ))),
            Command::Action(_) => {
                self.action_calls.fetch_add(1, Ordering::SeqCst);
                Ok(CommandResponse::Action(ExecuteActionResponse::new(
                    0,
                    "spy".to_string(),
                    String::new(),
                )))
            }
            _ => Err(CommandError::Transport(
                "spy received an unsupported command".to_string(),
            )),
        }
    }
}

fn sample_workflow_command() -> ExecuteWorkflowCommand {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join(".git")).unwrap();
    let repository = Repository::new(
        RepoPath::new(tmp.path().to_path_buf()).unwrap(),
        RepositoryName::new("test-repo".into()).unwrap(),
    );
    ExecuteWorkflowCommand::new(
        "name: CI\non: [push]\n".to_string(),
        WorkflowRunConfig::new(),
        repository,
        "test-run".to_string(),
        false,
    )
}

fn checkout_action_command() -> ExecuteActionCommand<dyn ContainerPort> {
    let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
        .unwrap()
        .into_domain();
    let container: Arc<dyn ContainerPort> = Arc::new(StubContainer);
    ExecuteActionCommand::new(
        "actions/checkout@v4".into(),
        step,
        PathBuf::from("/repo"),
        HashMap::new(),
        container,
    )
    .with_context(EvaluationContext::new())
}

#[test]
fn publisher_routes_workflow_command_to_workflow_handler() {
    let (workflow, job, step, action) = stub_handlers();
    let publisher = bound_publisher(workflow, job, step, action);

    let result = publisher
        .publish(Command::Workflow(sample_workflow_command()))
        .unwrap();
    let CommandResponse::Workflow(result) = result else {
        panic!("expected workflow response");
    };

    assert_eq!(result.workflow_name(), "dispatched-wf");
}

#[test]
fn publisher_routes_action_command_to_action_handler() {
    let (workflow, job, step, action) = stub_handlers();
    let publisher = bound_publisher(workflow, job, step, action);

    let result = publisher
        .publish(Command::Action(checkout_action_command()))
        .unwrap();
    let CommandResponse::Action(result) = result else {
        panic!("expected action response");
    };

    assert_eq!(result.stdout(), "action out");
    assert_eq!(result.exit_code(), 0);
}

#[test]
fn publisher_routes_workflow_command_without_reaching_action_handler() {
    let action_spy = SpyCommandHandler::default();
    let publisher = publisher_with_handler(Box::new(action_spy.clone()));

    let result = publisher
        .publish(Command::Workflow(sample_workflow_command()))
        .unwrap();
    let CommandResponse::Workflow(result) = result else {
        panic!("expected workflow response");
    };

    assert_eq!(result.workflow_name(), "dispatched-wf");
    assert_eq!(action_spy.action_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn action_handler_preserves_failed_action_output() {
    let handler = ActionCommandHandler::new(Box::new(|_| Box::new(FailingActionPort)));
    let container: Arc<dyn ContainerPort> = Arc::new(StubContainer);
    let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
        .unwrap()
        .into_domain();
    let command = ExecuteActionCommand::new(
        "actions/checkout@v4".into(),
        step,
        PathBuf::from("/repo"),
        HashMap::new(),
        container,
    )
    .with_context(EvaluationContext::new());

    let error = handler.handle(command).unwrap_err();

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
    let command = ExecuteJobCommand::new(
        Job::default(),
        "build-job".to_string(),
        workflow_named("Build"),
        repo_path.clone(),
        EvaluationContext::new(),
    )
    .with_run_id("test-run".to_string())
    .with_allow_repo_writes(false);

    let result = publisher.publish(Command::Job(Box::new(command))).unwrap();
    let CommandResponse::Job(result) = result else {
        panic!("expected job response");
    };

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
    let container: Arc<dyn ContainerPort> = Arc::new(StubContainer);
    let command = ExecuteStepCommand::new(
        step,
        HashMap::from([("MARKER".to_string(), "step-marker".to_string())]),
        EvaluationContext::new(),
        container,
        PathBuf::from("/repo/step"),
    );

    let result = publisher.publish(Command::Step(command)).unwrap();
    let CommandResponse::Step(result) = result else {
        panic!("expected step response");
    };

    assert_eq!(result.step().run(), Some("echo hello"));
    assert_eq!(result.response().stdout(), "step-marker");
    assert_eq!(result.response().stderr(), "/repo/step");
}
