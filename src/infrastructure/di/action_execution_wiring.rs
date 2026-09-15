use std::sync::Arc;

use crate::application::ports::outbound::{
    ActionCommandBusPort, DomainEventBusPort, LoadActionDefinitionPort, ResolveActionDirectoryPort,
    ResolveActionInputsPort, RunCompositeActionPort, RunNodeActionPort,
};
use crate::application::services::execute_action_service::ExecuteActionService;
use crate::infrastructure::actions::{
    ActionFetcherPort, CollectActionFilesService, CopyActionToContainerService,
    ExecuteActionFactory, FetchRemoteActionService, GitHubActionInputEnvironmentAdapter,
    LoadActionDefinitionService, ResolveActionDirectoryService, ResolveActionInputsService,
    ResolveNodeBinaryService, RunCompositeActionService, RunNodeActionService,
};
use crate::infrastructure::steps::{RunCompositeStepService, RunShellStepService};

pub struct ActionExecutionWiring;

impl ActionExecutionWiring {
    pub fn build(
        fetcher: Box<dyn ActionFetcherPort>,
        command_bus: Box<dyn ActionCommandBusPort>,
        event_bus: Box<dyn DomainEventBusPort>,
    ) -> ExecuteActionFactory {
        let directory_resolver: Arc<dyn ResolveActionDirectoryPort> = Arc::new(
            ResolveActionDirectoryService::new(Box::new(FetchRemoteActionService::new(fetcher))),
        );
        let definition_loader: Arc<dyn LoadActionDefinitionPort> =
            Arc::new(LoadActionDefinitionService::new());
        let input_resolver: Arc<dyn ResolveActionInputsPort> =
            Arc::new(ResolveActionInputsService::new());
        let composite_runner: Arc<dyn RunCompositeActionPort> = Arc::new(
            RunCompositeActionService::new(Box::new(RunCompositeStepService::new(
                Box::new(RunShellStepService::new(event_bus)),
                command_bus,
            ))),
        );
        let node_runner: Arc<dyn RunNodeActionPort> = Arc::new(RunNodeActionService::new(
            Box::new(CopyActionToContainerService::new(Box::new(
                CollectActionFilesService::new(),
            ))),
            Box::new(GitHubActionInputEnvironmentAdapter::new()),
            Box::new(ResolveNodeBinaryService::new()),
        ));

        Box::new(move |container| {
            Box::new(ExecuteActionService::new(
                container,
                directory_resolver.clone(),
                definition_loader.clone(),
                input_resolver.clone(),
                composite_runner.clone(),
                node_runner.clone(),
            ))
        })
    }
}
