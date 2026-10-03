use std::sync::Arc;

use crate::{
    actions::{
        CollectActionFilesService, CopyActionToContainerService, ExecuteActionFactory,
        FetchRemoteActionService, GitHubActionInputEnvironmentAdapter, LoadActionDefinitionService,
        ResolveActionDirectoryService, ResolveActionInputsService, ResolveNodeBinaryService,
        RunCompositeActionService, RunNodeActionService,
    },
    application::{
        ports::outbound::{
            ActionCommandPublisherPort, ActionDefinitionLoaderPort, ActionDirectoryResolverPort,
            ActionFetcherPort, ActionInputsResolverPort, CompositeActionRunnerPort,
            DomainEventPublisherPort, NodeActionRunnerPort, StepInterpolatorPort,
            StepTextCodecPort,
        },
        services::execute_action_service::{ExecuteActionDependencies, ExecuteActionService},
    },
    steps::{
        ReadStepEnvExportsService, ReadStepExportsService, ReadStepOutputExportsService,
        ReadStepPathExportsService, RunCompositeStepService, RunShellStepService,
    },
};

pub struct ActionExecutionWiring;

impl ActionExecutionWiring {
    pub fn build(
        fetcher: Box<dyn ActionFetcherPort>,
        command_publisher: Box<dyn ActionCommandPublisherPort>,
        event_bus: Box<dyn DomainEventPublisherPort>,
        step_codec: Arc<dyn StepTextCodecPort>,
        interpolator: Arc<dyn StepInterpolatorPort>,
    ) -> ExecuteActionFactory {
        let directory_resolver: Arc<dyn ActionDirectoryResolverPort> = Arc::new(
            ResolveActionDirectoryService::new(Box::new(FetchRemoteActionService::new(fetcher))),
        );
        let definition_loader: Arc<dyn ActionDefinitionLoaderPort> =
            Arc::new(LoadActionDefinitionService::new());
        let input_resolver: Arc<dyn ActionInputsResolverPort> =
            Arc::new(ResolveActionInputsService::new());
        let composite_runner =
            Self::build_composite_runner(event_bus, command_publisher, interpolator);
        let node_runner: Arc<dyn NodeActionRunnerPort> = Arc::new(RunNodeActionService::new(
            Box::new(CopyActionToContainerService::new(Box::new(
                CollectActionFilesService::new(),
            ))),
            Box::new(GitHubActionInputEnvironmentAdapter::new()),
            Box::new(ResolveNodeBinaryService::new()),
        ));
        let action_dependencies = ExecuteActionDependencies::new(
            (directory_resolver, definition_loader, input_resolver),
            (composite_runner, node_runner, step_codec),
        );

        Box::new(move |container| {
            Box::new(ExecuteActionService::new(
                container,
                action_dependencies.clone(),
            ))
        })
    }
    fn build_composite_runner(
        event_bus: Box<dyn DomainEventPublisherPort>,
        command_publisher: Box<dyn ActionCommandPublisherPort>,
        interpolator: Arc<dyn StepInterpolatorPort>,
    ) -> Arc<dyn CompositeActionRunnerPort> {
        let composite_exports_reader = Box::new(ReadStepExportsService::new(
            Box::new(ReadStepPathExportsService::new()),
            Box::new(ReadStepEnvExportsService::new()),
            Box::new(ReadStepOutputExportsService::new()),
        ));
        Arc::new(RunCompositeActionService::new(
            Box::new(RunCompositeStepService::new(
                Box::new(RunShellStepService::new(event_bus)),
                command_publisher,
            )),
            Box::new(CopyActionToContainerService::new(Box::new(
                CollectActionFilesService::new(),
            ))),
            composite_exports_reader,
            interpolator,
        ))
    }
}
