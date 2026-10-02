use std::sync::Arc;

use crate::{
    actions::ActionCommandHandler,
    application::{
        ports::outbound::{
            ActionFetcherPort, CommandPublisherPort, ContainerRuntimePort, ImageMapperPort,
            StepTextCodecPort,
        },
        services::{
            execute_job_service::{ExecuteJobDependencies, ExecuteJobService},
            execute_step_service::ExecuteStepService,
        },
    },
    containers::{
        CreateJobContainerService, PrepareJobContainerService, PullJobImageService,
        RepositoryContainerCopyAdapter,
    },
    di::action_execution_wiring::ActionExecutionWiring,
    jobs::{JobCommandHandler, RunnerEnvironmentAdapter},
    messaging::{
        CommandHandlerAdapter, CommandPublisherAdapter, DeferredCommandBus,
        DomainEventPublisherAdapter, InMemoryCommandBus,
    },
    steps::{
        ExecuteStepFactory, FragmentNetworkCommandClassifier, JsonStepTextCodec,
        StepCommandHandler, build_step_context_service::BuildStepContextService,
        prefix_step_path_service::PrefixStepPathService,
        read_step_env_exports_service::ReadStepEnvExportsService,
        read_step_exports_service::ReadStepExportsService,
        read_step_path_exports_service::ReadStepPathExportsService,
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
        image_mapper: Box<dyn ImageMapperPort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        event_bus: DomainEventPublisherAdapter,
    ) -> CommandPublisherAdapter {
        let image_mapper: Arc<dyn ImageMapperPort> = Arc::from(image_mapper);
        let deferred = Arc::new(DeferredCommandBus::new());
        let publisher = CommandPublisherAdapter::new(deferred.clone());

        let workflow_handler = WorkflowCommandHandler::new(Box::new(ExecuteWorkflowService::new(
            Box::new(LoadWorkflowService::new()),
            Box::new(publisher.clone()) as Box<dyn CommandPublisherPort>,
            Box::new(event_bus.clone()),
        )));

        let job_handler = JobCommandHandler::new(Box::new(Self::build_job_executor(
            runtime.clone(),
            image_mapper,
            Box::new(publisher.clone()) as Box<dyn CommandPublisherPort>,
            Box::new(event_bus.clone()),
        )));

        let (step_handler, step_codec) = Self::build_step_handler(&event_bus, &publisher);

        let action_factory = ActionExecutionWiring::build(
            action_fetcher,
            Box::new(publisher.clone()) as Box<dyn CommandPublisherPort>,
            Box::new(event_bus),
            step_codec,
        );
        let action_handler = ActionCommandHandler::new(action_factory);
        let command_handler =
            CommandHandlerAdapter::new(workflow_handler, job_handler, step_handler, action_handler);

        deferred.bind(InMemoryCommandBus::new(Box::new(command_handler)));

        publisher
    }

    fn build_step_handler(
        event_bus: &DomainEventPublisherAdapter,
        publisher: &CommandPublisherAdapter,
    ) -> (StepCommandHandler, Arc<dyn StepTextCodecPort>) {
        let step_codec: Arc<dyn StepTextCodecPort> = Arc::new(JsonStepTextCodec);
        let shell_runner = Arc::new(RunShellStepService::new(Box::new(event_bus.clone())));
        let action_publisher = Arc::new(publisher.clone()) as Arc<dyn CommandPublisherPort>;
        let step_codec_for_factory = step_codec.clone();
        let step_factory: ExecuteStepFactory = Box::new(move |container| {
            Box::new(ExecuteStepService::new(
                container,
                shell_runner.clone(),
                action_publisher.clone(),
                step_codec_for_factory.clone(),
            ))
        });
        (StepCommandHandler::new(step_factory), step_codec)
    }

    fn build_job_executor(
        runtime: Arc<dyn ContainerRuntimePort>,
        image_mapper: Arc<dyn ImageMapperPort>,
        command_bus: Box<dyn CommandPublisherPort>,
        event_bus: Box<dyn crate::application::ports::outbound::DomainEventPublisherPort>,
    ) -> ExecuteJobService {
        ExecuteJobService::new(ExecuteJobDependencies::new(
            Box::new(RunnerEnvironmentAdapter::new()),
            Box::new(PrepareJobContainerService::new(
                Box::new(PullJobImageService::new(
                    runtime.clone(),
                    image_mapper.clone(),
                )),
                Box::new(CreateJobContainerService::new(runtime.clone())),
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
                )),
            ),
            (command_bus, event_bus),
        ))
    }
}
