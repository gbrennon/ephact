use std::sync::Arc;

use ephact::{
    application::ports::outbound::{ContainerRuntimePort, WorkflowSourcePort},
    infrastructure::{
        actions::ActionFetcherPort, di::Container, persistence::CargoProjectBrandingStore,
    },
    presentation::composition_root::{Application, CompositionRoot},
};

use crate::e2e_fixed_image_mapper::FixedImageMapper;

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
        );
        let container = Container::with_collaborators_and_branding(
            runtime,
            Box::new(FixedImageMapper),
            fetcher,
            workflow_source,
            None,
            Box::new(branding_store),
        );

        CompositionRoot::compose(container)
    }
}
