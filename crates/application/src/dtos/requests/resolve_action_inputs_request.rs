use crate::domain::{entities::Step, value_objects::ActionDefinition};

/// Request data for resolving action inputs.
pub struct ResolveActionInputsRequest {
    definition: ActionDefinition,
    step: Step,
}

impl ResolveActionInputsRequest {
    /// Creates a request from an action definition and step.
    pub fn new(definition: ActionDefinition, step: Step) -> Self {
        Self { definition, step }
    }

    /// Returns the action definition.
    pub fn definition(&self) -> &ActionDefinition {
        &self.definition
    }

    /// Returns the step.
    pub fn step(&self) -> &Step {
        &self.step
    }
}
