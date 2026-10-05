use std::sync::Arc;

use crate::application::ports::outbound::{
    ActionFetcherPort, ContainerRuntimePort, WorkflowSourcePort,
};

/// Groups the pluggable infrastructure components required to assemble a container.
pub struct ContainerCollaborators {
    runtime: Arc<dyn ContainerRuntimePort>,
    action_fetcher: Box<dyn ActionFetcherPort>,
    workflow_source: Arc<dyn WorkflowSourcePort>,
}

impl ContainerCollaborators {
    /// Creates a set of collaborators from the supplied ports.
    pub fn new(
        runtime: Arc<dyn ContainerRuntimePort>,
        action_fetcher: Box<dyn ActionFetcherPort>,
        workflow_source: Arc<dyn WorkflowSourcePort>,
    ) -> Self {
        Self {
            runtime,
            action_fetcher,
            workflow_source,
        }
    }
    /// Consumes the collaborators, returning their ports for composition.
    pub fn into_parts(
        self,
    ) -> (
        Arc<dyn ContainerRuntimePort>,
        Box<dyn ActionFetcherPort>,
        Arc<dyn WorkflowSourcePort>,
    ) {
        (self.runtime, self.action_fetcher, self.workflow_source)
    }
}
