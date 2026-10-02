use std::sync::Arc;

use crate::{
    actions::{GitActionFetcher, RunActionFactory},
    application::{
        ports::outbound::{
            ActionFetcherPort, CommandPublisherPort, ContainerRuntimePort, ImageMapperPort,
            ProjectBrandingStorePort, WorkflowSourcePort,
            domain_event_handler_port::DomainEventHandlerPort,
            domain_event_publisher_port::DomainEventPublisherPort,
        },
        services::{
            list_actions_service::ListActionsService, list_workflows_service::ListWorkflowsService,
            run_action_service::RunActionService,
            show_project_branding_info_service::ShowProjectBrandingInfoService,
        },
    },
    containers::{ContainerCleanupHandler, ContainerRuntimeAdapter},
    di::{
        app_container::{AppContainer, AppContainerParts},
        command_bus_wiring::CommandBusWiring,
    },
    images::PlatformImageMapper,
    logging::{FailureLogErrorStore, FailureLogHandler, FailureLogPathStore, FailureLogStores},
    messaging::{CommandPublisherAdapter, DomainEventPublisherAdapter, InMemoryEventBus},
    steps::JsonStepTextCodec,
    workflows::{
        DetectWorkflowTriggerService, FilesystemRunInputDiscoveryService, FilesystemWorkflowSource,
        RunAllWorkflowsService, RunWorkflowService, SharedWorkflowSource,
    },
};

/// Groups the pluggable infrastructure components required to assemble a container.
pub struct ContainerCollaborators {
    runtime: Arc<dyn ContainerRuntimePort>,
    image_mapper: Box<dyn ImageMapperPort>,
    action_fetcher: Box<dyn ActionFetcherPort>,
    workflow_source: Arc<dyn WorkflowSourcePort>,
}

impl ContainerCollaborators {
    /// Creates a set of collaborators from the supplied ports.
    pub fn new(
        runtime: Arc<dyn ContainerRuntimePort>,
        image_mapper: Box<dyn ImageMapperPort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        workflow_source: Arc<dyn WorkflowSourcePort>,
    ) -> Self {
        Self {
            runtime,
            image_mapper,
            action_fetcher,
            workflow_source,
        }
    }
}

pub struct Container {}

impl Container {
    pub fn build_with_branding(
        progress_reporter: Option<Box<dyn DomainEventHandlerPort>>,
        branding_store: Box<dyn ProjectBrandingStorePort>,
    ) -> AppContainer {
        let runtime: Arc<dyn ContainerRuntimePort> = Arc::new(
            ContainerRuntimeAdapter::detect()
                .expect("no container runtime available (Docker or Podman required)"),
        );
        Self::with_collaborators_and_branding(
            ContainerCollaborators::new(
                runtime,
                Box::new(PlatformImageMapper),
                Box::new(GitActionFetcher::with_default_cache_root()),
                Arc::new(FilesystemWorkflowSource::default()),
            ),
            progress_reporter,
            branding_store,
        )
    }

    pub fn with_collaborators_and_branding(
        collaborators: ContainerCollaborators,
        progress_reporter: Option<Box<dyn DomainEventHandlerPort>>,
        branding_store: Box<dyn ProjectBrandingStorePort>,
    ) -> AppContainer {
        let ContainerCollaborators {
            runtime,
            image_mapper,
            action_fetcher,
            workflow_source,
        } = collaborators;
        let (event_publisher, failure_log_stores) =
            Self::build_event_bus(runtime.clone(), progress_reporter);
        let shared_workflow_source = SharedWorkflowSource::new(workflow_source);
        let command_publisher = Self::build_command_bus(
            runtime,
            image_mapper,
            action_fetcher,
            event_publisher.clone(),
        );
        let parts = Self::build_app_parts(
            shared_workflow_source,
            command_publisher,
            event_publisher,
            branding_store,
        );
        AppContainer::new_with_discovery_and_failure_stores(parts, failure_log_stores)
    }

    fn build_app_parts(
        workflow_source: SharedWorkflowSource,
        command_publisher: CommandPublisherAdapter,
        event_publisher: DomainEventPublisherAdapter,
        branding_store: Box<dyn ProjectBrandingStorePort>,
    ) -> AppContainerParts {
        let list_workflows_service = ListWorkflowsService::new(Box::new(workflow_source.clone()));
        let list_actions_service = ListActionsService::new(Box::new(workflow_source.clone()));
        let run_workflow_service = RunWorkflowService::new(
            Box::new(workflow_source.clone()),
            Box::new(command_publisher.clone()) as Box<dyn CommandPublisherPort>,
            Box::new(event_publisher.clone()) as Box<dyn DomainEventPublisherPort>,
            Box::new(DetectWorkflowTriggerService::new()),
        );
        let discover_run_inputs_service =
            FilesystemRunInputDiscoveryService::new(Box::new(workflow_source.clone()));
        let run_all_workflows_service = RunAllWorkflowsService::new(
            Box::new(workflow_source),
            Box::new(command_publisher.clone()) as Box<dyn CommandPublisherPort>,
            Box::new(event_publisher) as Box<dyn DomainEventPublisherPort>,
            Box::new(DetectWorkflowTriggerService::new()),
        );
        let run_action_factory: RunActionFactory = Box::new(move |container| {
            Box::new(RunActionService::new(
                Box::new(command_publisher.clone()),
                container,
                Arc::new(JsonStepTextCodec),
            ))
        });
        let show_project_branding_info_service =
            ShowProjectBrandingInfoService::new(branding_store);

        (
            Box::new(show_project_branding_info_service),
            Box::new(run_all_workflows_service),
            Box::new(run_workflow_service),
            run_action_factory,
            Box::new(discover_run_inputs_service),
            Box::new(list_workflows_service),
            Box::new(list_actions_service),
        )
    }

    fn build_event_bus(
        runtime: Arc<dyn ContainerRuntimePort>,
        progress_reporter: Option<Box<dyn DomainEventHandlerPort>>,
    ) -> (DomainEventPublisherAdapter, FailureLogStores) {
        let failure_log_error_store = FailureLogErrorStore::new();
        let failure_log_path_store = FailureLogPathStore::new();
        let failure_log_handler = FailureLogHandler::with_temp_root_and_stores(
            std::env::temp_dir(),
            failure_log_error_store.clone(),
            failure_log_path_store.clone(),
        );
        let mut handlers: Vec<Box<dyn DomainEventHandlerPort>> = vec![
            Box::new(ContainerCleanupHandler::new(runtime)),
            Box::new(failure_log_handler),
        ];
        if let Some(reporter) = progress_reporter {
            handlers.push(reporter);
        }
        let in_memory_event_bus = Arc::new(InMemoryEventBus::new(handlers));
        (
            DomainEventPublisherAdapter::new(in_memory_event_bus),
            FailureLogStores::from_stores(failure_log_error_store, failure_log_path_store),
        )
    }

    fn build_command_bus(
        runtime: Arc<dyn ContainerRuntimePort>,
        image_mapper: Box<dyn ImageMapperPort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        event_bus: DomainEventPublisherAdapter,
    ) -> CommandPublisherAdapter {
        CommandBusWiring::build(runtime, image_mapper, action_fetcher, event_bus)
    }
}
