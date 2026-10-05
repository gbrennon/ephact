use crate::{
    domain::traits::NetworkCommandClassifier,
    ports::outbound::{
        DomainEventPublisherPort, StepCommandPublisherPort,
        job_container_preparer_port::JobContainerPreparerPort,
        job_environment_builder_port::JobEnvironmentBuilderPort,
        step_context_builder_port::StepContextBuilderPort,
        step_exports_reader_port::StepExportsReaderPort,
        step_path_prefixer_port::StepPathPrefixerPort, step_summarizer_port::StepSummarizerPort,
    },
};

/// Dependencies used by the job execution application service for each step.
pub type ExecuteJobStepDependencies = (
    Box<dyn StepPathPrefixerPort>,
    Box<dyn StepContextBuilderPort>,
    Box<dyn StepSummarizerPort>,
    Box<dyn StepExportsReaderPort>,
);

/// Dependencies used by the job execution application service for messaging.
pub type ExecuteJobMessagingDependencies = (
    Box<dyn StepCommandPublisherPort>,
    Box<dyn DomainEventPublisherPort>,
);
/// Owned dependencies in the order required by job execution.
pub type ExecuteJobDependenciesParts = (
    Box<dyn JobEnvironmentBuilderPort>,
    Box<dyn JobContainerPreparerPort>,
    Box<dyn StepPathPrefixerPort>,
    Box<dyn StepContextBuilderPort>,
    Box<dyn StepSummarizerPort>,
    Box<dyn StepExportsReaderPort>,
    Box<dyn StepCommandPublisherPort>,
    Box<dyn DomainEventPublisherPort>,
    Box<dyn NetworkCommandClassifier>,
);

/// Borrowed dependencies in the order required by job execution.
pub type ExecuteJobDependenciesRefs<'a> = (
    &'a dyn JobEnvironmentBuilderPort,
    &'a dyn JobContainerPreparerPort,
    &'a dyn StepPathPrefixerPort,
    &'a dyn StepContextBuilderPort,
    &'a dyn StepSummarizerPort,
    &'a dyn StepExportsReaderPort,
    &'a dyn StepCommandPublisherPort,
    &'a dyn DomainEventPublisherPort,
    &'a dyn NetworkCommandClassifier,
);

/// Dependencies required to construct the job execution application service.
pub struct ExecuteJobDependencies {
    job_environment_builder: Box<dyn JobEnvironmentBuilderPort>,
    container_preparer: Box<dyn JobContainerPreparerPort>,
    step_path_prefixer: Box<dyn StepPathPrefixerPort>,
    step_context_builder: Box<dyn StepContextBuilderPort>,
    step_summarizer: Box<dyn StepSummarizerPort>,
    step_exports_reader: Box<dyn StepExportsReaderPort>,
    command_bus: Box<dyn StepCommandPublisherPort>,
    event_bus: Box<dyn DomainEventPublisherPort>,
    network_command_classifier: Box<dyn NetworkCommandClassifier>,
}

impl ExecuteJobDependencies {
    /// Creates the dependency bundle used by [`super::ExecuteJobService`].
    pub fn new(
        job_environment_builder: Box<dyn JobEnvironmentBuilderPort>,
        container_preparer: Box<dyn JobContainerPreparerPort>,
        network_command_classifier: Box<dyn NetworkCommandClassifier>,
        step_dependencies: ExecuteJobStepDependencies,
        messaging_dependencies: ExecuteJobMessagingDependencies,
    ) -> Self {
        let (step_path_prefixer, step_context_builder, step_summarizer, step_exports_reader) =
            step_dependencies;
        let (command_bus, event_bus) = messaging_dependencies;
        Self {
            job_environment_builder,
            container_preparer,
            step_path_prefixer,
            step_context_builder,
            step_summarizer,
            step_exports_reader,
            command_bus,
            event_bus,
            network_command_classifier,
        }
    }

    /// Consumes the bundle and returns its dependencies in service field order.
    pub fn into_parts(self) -> ExecuteJobDependenciesParts {
        (
            self.job_environment_builder,
            self.container_preparer,
            self.step_path_prefixer,
            self.step_context_builder,
            self.step_summarizer,
            self.step_exports_reader,
            self.command_bus,
            self.event_bus,
            self.network_command_classifier,
        )
    }
    /// Borrows the bundle's dependencies for one job execution.
    pub fn as_parts(&self) -> ExecuteJobDependenciesRefs<'_> {
        (
            self.job_environment_builder.as_ref(),
            self.container_preparer.as_ref(),
            self.step_path_prefixer.as_ref(),
            self.step_context_builder.as_ref(),
            self.step_summarizer.as_ref(),
            self.step_exports_reader.as_ref(),
            self.command_bus.as_ref(),
            self.event_bus.as_ref(),
            self.network_command_classifier.as_ref(),
        )
    }
}
