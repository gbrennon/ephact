use crate::domain::{entities::Step, value_objects::ActionDefinition};

/// Request data for resolving action inputs.
pub struct ResolveActionInputsRequest {
    definition: ActionDefinition,
    step: Step,
}

impl ResolveActionInputsRequest {
    pub fn new(definition: ActionDefinition, step: Step) -> Self {
        Self { definition, step }
    }

    pub fn definition(&self) -> &ActionDefinition {
        &self.definition
    }

    pub fn step(&self) -> &Step {
        &self.step
    }
}
