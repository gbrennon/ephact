use std::sync::Arc;

use crate::{
    application::{
        ports::outbound::{
            ContainerRuntimePort, WorkflowSourcePort, command_bus_port::ActionCommandBusPort,
            command_bus_port::WorkflowCommandBusPort, event_bus_port::DomainEventBusPort,
        },
        services::{
            list_actions_service::ListActionsService, list_workflows_service::ListWorkflowsService,
            run_action_service::RunActionService,
            run_all_workflows_service::RunAllWorkflowsService,
            run_workflow_service::RunWorkflowService,
            show_project_branding_info_service::ShowProjectBrandingInfoService,
        },
    },
    infrastructure::{
        actions::{ActionFetcherPort, GitActionFetcher},
        containers::{ContainerCleanupHandler, ContainerRuntimeAdapter},
        di::{app_container::AppContainer, command_bus_wiring::CommandBusWiring},
        images::{ImageMapperPort, PlatformImageMapper},
        messaging::{DomainEventHandler, InMemoryEventBus, SharedEventBus},
        project_branding_store::CargoProjectBrandingStore,
        workflows::{
            DetectWorkflowTriggerService, FilesystemRunInputDiscoveryService,
            FilesystemWorkflowSource, SharedWorkflowSource,
        },
    },
};

pub struct Container;

impl Container {
    pub fn build(progress_reporter: Option<Box<dyn DomainEventHandler>>) -> AppContainer {
        let runtime: Arc<dyn ContainerRuntimePort> = Arc::new(
            ContainerRuntimeAdapter::detect()
                .expect("no container runtime available (Docker or Podman required)"),
        );
        Self::with_runtime(runtime, progress_reporter)
    }

    pub fn with_runtime(
        runtime: Arc<dyn ContainerRuntimePort>,
        progress_reporter: Option<Box<dyn DomainEventHandler>>,
    ) -> AppContainer {
        Self::with_collaborators(
            runtime,
            Box::new(PlatformImageMapper),
            Box::new(GitActionFetcher::with_default_cache_root()),
            Arc::new(FilesystemWorkflowSource::default()),
            progress_reporter,
        )
    }

    pub fn with_collaborators(
        runtime: Arc<dyn ContainerRuntimePort>,
        image_mapper: Box<dyn ImageMapperPort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        workflow_source: Arc<dyn WorkflowSourcePort>,
        progress_reporter: Option<Box<dyn DomainEventHandler>>,
    ) -> AppContainer {
        let shared_workflow_source = SharedWorkflowSource::new(workflow_source);

        let mut handlers: Vec<Box<dyn DomainEventHandler>> =
            vec![Box::new(ContainerCleanupHandler::new(runtime.clone()))];
        if let Some(reporter) = progress_reporter {
            handlers.push(reporter);
        }
        let in_memory_event_bus = Arc::new(InMemoryEventBus::new(handlers));
        let shared_event_bus = SharedEventBus::new(in_memory_event_bus);

        let shared_command_bus = CommandBusWiring::build(
            runtime,
            image_mapper,
            action_fetcher,
            shared_event_bus.clone(),
        );

        let list_workflows_service =
            ListWorkflowsService::new(Box::new(shared_workflow_source.clone()));
        let list_actions_service =
            ListActionsService::new(Box::new(shared_workflow_source.clone()));

        let run_workflow_service = RunWorkflowService::new(
            Box::new(shared_workflow_source.clone()),
            Box::new(shared_command_bus.clone()) as Box<WorkflowCommandBusPort>,
            Box::new(shared_event_bus.clone()) as Box<DomainEventBusPort>,
            Box::new(DetectWorkflowTriggerService::new()),
        );
        let discover_run_inputs_service =
            FilesystemRunInputDiscoveryService::new(Box::new(shared_workflow_source.clone()));
        let run_all_workflows_service = RunAllWorkflowsService::new(
            Box::new(shared_workflow_source),
            Box::new(shared_command_bus.clone()) as Box<WorkflowCommandBusPort>,
            Box::new(shared_event_bus) as Box<DomainEventBusPort>,
            Box::new(DetectWorkflowTriggerService::new()),
        );
        let run_action_service =
            RunActionService::new(Box::new(shared_command_bus) as Box<ActionCommandBusPort>);
        let show_project_branding_info_service =
            ShowProjectBrandingInfoService::new(Box::new(CargoProjectBrandingStore));

        AppContainer::new_with_discovery(
            Box::new(show_project_branding_info_service),
            Box::new(run_all_workflows_service),
            Box::new(run_workflow_service),
            Box::new(run_action_service),
            Box::new(discover_run_inputs_service),
            Box::new(list_workflows_service),
            Box::new(list_actions_service),
        )
    }
}
