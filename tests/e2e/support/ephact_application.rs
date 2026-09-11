use std::sync::Arc;

use ephact::{
    application::ports::outbound::{ContainerRuntimePort, WorkflowSourcePort},
    infrastructure::{actions::ActionFetcherPort, di::Container},
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
        let container = Container::with_collaborators(
            runtime,
            Box::new(FixedImageMapper),
            fetcher,
            workflow_source,
            None,
        );

        CompositionRoot::compose(container)
    }
}
