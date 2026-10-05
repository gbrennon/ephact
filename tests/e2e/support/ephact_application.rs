use std::sync::Arc;

use ephact::{
    application::ports::outbound::{ActionFetcherPort, ContainerRuntimePort, WorkflowSourcePort},
    infrastructure::{
        actions::GitActionFetcher,
        di::{Container, container::ContainerCollaborators},
        persistence::CargoProjectBrandingStore,
    },
    presentation::composition_root::{Application, CompositionRoot},
};

pub struct EphactApplication;

impl EphactApplication {
    pub fn compose(
        runtime: Arc<dyn ContainerRuntimePort>,
        fetcher: Box<dyn ActionFetcherPort>,
        workflow_source: Arc<dyn WorkflowSourcePort>,
    ) -> Application {
        let branding_store = CargoProjectBrandingStore::from_metadata(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            ephact::PROJECT_EMBLEM,
        );
        let container = Container::with_collaborators_and_branding(
            ContainerCollaborators::new(runtime, fetcher, workflow_source),
            None,
            Box::new(branding_store),
        );

        CompositionRoot::compose(container)
    }

    pub fn compose_with_production_adapters(
        runtime: Arc<dyn ContainerRuntimePort>,
        workflow_source: Arc<dyn WorkflowSourcePort>,
    ) -> Application {
        let branding_store = CargoProjectBrandingStore::from_metadata(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            ephact::PROJECT_EMBLEM,
        );
        let container = Container::with_collaborators_and_branding(
            ContainerCollaborators::new(
                runtime,
                Box::new(GitActionFetcher::with_default_cache_root()),
                workflow_source,
            ),
            None,
            Box::new(branding_store),
        );

        CompositionRoot::compose(container)
    }
}
