use std::sync::Arc;

use crate::{
    application::{
        ports::outbound::{
            ContainerRuntimePort, action_command_bus_port::ActionCommandBusPort,
            domain_event_bus_port::DomainEventBusPort, job_command_bus_port::JobCommandBusPort,
            step_command_bus_port::StepCommandBusPort,
        },
        services::{
            execute_job_service::{ExecuteJobDependencies, ExecuteJobService},
            execute_step_service::ExecuteStepService,
            execute_workflow_service::ExecuteWorkflowService,
        },
    },
    infrastructure::{
        actions::{ActionCommandHandler, ActionFetcherPort},
        containers::{
            CreateJobContainerService, PrepareJobContainerService, PullJobImageService,
            RepositoryContainerCopyAdapter,
        },
        di::action_execution_wiring::ActionExecutionWiring,
        images::ImageMapperPort,
        jobs::{JobCommandHandler, RunnerEnvironmentAdapter},
        messaging::{DeferredCommandBus, InMemoryCommandBus, SharedCommandBus, SharedEventBus},
        steps::{
            ExecuteStepFactory, StepCommandHandler,
            build_step_context_service::BuildStepContextService,
            prefix_step_path_service::PrefixStepPathService,
            read_step_env_exports_service::ReadStepEnvExportsService,
            read_step_exports_service::ReadStepExportsService,
            read_step_path_exports_service::ReadStepPathExportsService,
            run_shell_step_service::RunShellStepService,
            summarize_step_service::SummarizeStepService,
        },
        workflows::{WorkflowCommandHandler, load_workflow_service::LoadWorkflowService},
    },
};

/// Assembles the command bus and the handler graph behind it.
///
/// Every coordination step of a run is a command: the workflow service
/// publishes job commands, the job service publishes step commands, and the
/// step service publishes action commands. Handlers are the only components
/// that know both a command and the application service handling it, so the
/// services are handed a [`DeferredCommandBus`] and the assembled bus is bound
/// into it once the graph is complete.
pub struct CommandBusWiring;

impl CommandBusWiring {
    #[must_use]
    pub fn build(
        runtime: Arc<dyn ContainerRuntimePort>,
        image_mapper: Box<dyn ImageMapperPort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        event_bus: SharedEventBus,
    ) -> SharedCommandBus {
        let image_mapper: Arc<dyn ImageMapperPort> = Arc::from(image_mapper);
        let deferred = Arc::new(DeferredCommandBus::new());
        let shared_bus = SharedCommandBus::new(deferred.clone());

        let workflow_handler = WorkflowCommandHandler::new(Box::new(ExecuteWorkflowService::new(
            Box::new(LoadWorkflowService::new()),
            Box::new(shared_bus.clone()) as Box<dyn JobCommandBusPort>,
            Box::new(event_bus.clone()) as Box<dyn DomainEventBusPort>,
        )));

        let job_handler = JobCommandHandler::new(Box::new(Self::build_job_executor(
            runtime.clone(),
            image_mapper,
            Box::new(shared_bus.clone()) as Box<dyn StepCommandBusPort>,
            Box::new(event_bus.clone()) as Box<dyn DomainEventBusPort>,
        )));

        let shell_runner = Arc::new(RunShellStepService::new(
            Box::new(event_bus.clone()) as Box<dyn DomainEventBusPort>
        ));
        let action_command_bus = Arc::new(shared_bus.clone()) as Arc<dyn ActionCommandBusPort>;
        let step_factory: ExecuteStepFactory = Box::new(move |container| {
            Box::new(ExecuteStepService::new(
                container,
                shell_runner.clone(),
                action_command_bus.clone(),
            ))
        });
        let step_handler = StepCommandHandler::new(step_factory);

        let action_factory = ActionExecutionWiring::build(
            action_fetcher,
            Box::new(shared_bus.clone()) as Box<dyn ActionCommandBusPort>,
            Box::new(event_bus) as Box<dyn DomainEventBusPort>,
        );
        let action_handler = ActionCommandHandler::new(action_factory);

        deferred.bind(InMemoryCommandBus::new(
            Box::new(workflow_handler),
            Box::new(job_handler),
            Box::new(step_handler),
            Box::new(action_handler),
        ));

        shared_bus
    }

    fn build_job_executor(
        runtime: Arc<dyn ContainerRuntimePort>,
        image_mapper: Arc<dyn ImageMapperPort>,
        command_bus: Box<dyn StepCommandBusPort>,
        event_bus: Box<dyn DomainEventBusPort>,
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
