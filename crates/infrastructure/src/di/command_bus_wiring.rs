use std::sync::Arc;

use crate::{
    actions::ActionCommandHandler,
    application::{
        ports::outbound::{
            ActionCommandPublisherPort, ActionFetcherPort, ContainerRuntimePort,
            DomainEventPublisherPort, JobCommandPublisherPort, StepCommandPublisherPort,
            StepInterpolatorPort, StepTextCodecPort,
        },
        services::{
            ExecuteJobDependencies, ExecuteJobService, execute_step_service::ExecuteStepService,
        },
    },
    containers::{
        CreateJobContainerService, GITHUB_HOSTED_RUNNER_IMAGE, HostSshForwardingConfig,
        PrepareJobContainerService, PullJobImageService, RepositoryContainerCopyAdapter,
    },
    di::action_execution_wiring::ActionExecutionWiring,
    jobs::{JobCommandHandler, RunnerEnvironmentAdapter},
    messaging::{CommandPublisherAdapter, DeferredCommandBus, InMemoryCommandBus},
    steps::{
        ExecuteStepFactory, FragmentNetworkCommandClassifier, JsonStepTextCodec,
        ReadStepEnvExportsService, ReadStepExportsService, ReadStepOutputExportsService,
        ReadStepPathExportsService, StepCommandHandler, StepInterpolator,
        build_step_context_service::BuildStepContextService,
        prefix_step_path_service::PrefixStepPathService,
        run_shell_step_service::RunShellStepService, summarize_step_service::SummarizeStepService,
    },
    workflows::{
        ExecuteWorkflowService, WorkflowCommandHandler, load_workflow_service::LoadWorkflowService,
    },
};

/// Assembles the command bus and the handler graph behind it.
pub struct CommandBusWiring;

impl CommandBusWiring {
    #[must_use]
    pub fn build(
        runtime: Arc<dyn ContainerRuntimePort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        event_bus: crate::messaging::DomainEventPublisherAdapter,
    ) -> CommandPublisherAdapter {
        Self::build_with_ssh_forwarding(
            runtime,
            action_fetcher,
            event_bus,
            HostSshForwardingConfig::disabled(),
        )
    }

    /// Builds the command bus with explicit host SSH-agent forwarding configuration.
    pub fn build_with_ssh_forwarding(
        runtime: Arc<dyn ContainerRuntimePort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        event_bus: crate::messaging::DomainEventPublisherAdapter,
        ssh_forwarding: HostSshForwardingConfig,
    ) -> CommandPublisherAdapter {
        let deferred = Arc::new(DeferredCommandBus::new());
        let publisher = CommandPublisherAdapter::new(deferred.clone());

        let workflow_handler = WorkflowCommandHandler::new(Box::new(ExecuteWorkflowService::new(
            Box::new(LoadWorkflowService::new()),
            Box::new(publisher.clone()) as Box<dyn JobCommandPublisherPort>,
            Box::new(event_bus.clone()),
        )));

        let job_handler = JobCommandHandler::new(Box::new(Self::build_job_executor(
            runtime.clone(),
            Box::new(publisher.clone()) as Box<dyn StepCommandPublisherPort>,
            Box::new(event_bus.clone()),
            ssh_forwarding,
        )));

        let interpolator: Arc<dyn StepInterpolatorPort> = Arc::new(StepInterpolator);
        let (step_handler, step_codec) =
            Self::build_step_handler(&event_bus, &publisher, interpolator.clone());

        let action_factory = ActionExecutionWiring::build(
            action_fetcher,
            Box::new(publisher.clone()) as Box<dyn ActionCommandPublisherPort>,
            Box::new(event_bus),
            step_codec,
            interpolator,
        );
        let action_handler = ActionCommandHandler::new(action_factory);

        deferred.bind(InMemoryCommandBus::new(
            Box::new(workflow_handler),
            Box::new(job_handler),
            Box::new(step_handler),
            Box::new(action_handler),
        ));

        publisher
    }

    fn build_step_handler(
        event_bus: &crate::messaging::DomainEventPublisherAdapter,
        publisher: &CommandPublisherAdapter,
        interpolator: Arc<dyn StepInterpolatorPort>,
    ) -> (StepCommandHandler, Arc<dyn StepTextCodecPort>) {
        let step_codec: Arc<dyn StepTextCodecPort> = Arc::new(JsonStepTextCodec);
        let shell_runner = Arc::new(RunShellStepService::new(Box::new(event_bus.clone())));
        let action_publisher = Arc::new(publisher.clone()) as Arc<dyn ActionCommandPublisherPort>;
        let step_codec_for_factory = step_codec.clone();
        let step_factory: ExecuteStepFactory = Box::new(move |container| {
            Box::new(ExecuteStepService::new(
                container,
                shell_runner.clone(),
                action_publisher.clone(),
                step_codec_for_factory.clone(),
                interpolator.clone(),
            ))
        });
        (StepCommandHandler::new(step_factory), step_codec)
    }

    fn build_job_executor(
        runtime: Arc<dyn ContainerRuntimePort>,
        command_bus: Box<dyn StepCommandPublisherPort>,
        event_bus: Box<dyn DomainEventPublisherPort>,
        ssh_forwarding: HostSshForwardingConfig,
    ) -> ExecuteJobService {
        ExecuteJobService::new(ExecuteJobDependencies::new(
            Box::new(RunnerEnvironmentAdapter::new()),
            Box::new(PrepareJobContainerService::with_default_image(
                GITHUB_HOSTED_RUNNER_IMAGE,
                Box::new(PullJobImageService::new(runtime.clone())),
                Box::new(CreateJobContainerService::with_ssh_forwarding(
                    runtime.clone(),
                    ssh_forwarding,
                )),
                Box::new(RepositoryContainerCopyAdapter::new()),
            )),
            Box::new(FragmentNetworkCommandClassifier::new()),
            (
                Box::new(PrefixStepPathService::new()),
                Box::new(BuildStepContextService::new()),
                Box::new(SummarizeStepService::new()),
                Box::new(ReadStepExportsService::new(
                    Box::new(ReadStepPathExportsService::new()),
                    Box::new(ReadStepEnvExportsService::new()),
                    Box::new(ReadStepOutputExportsService::new()),
                )),
            ),
            (command_bus, event_bus),
        ))
    }
}
