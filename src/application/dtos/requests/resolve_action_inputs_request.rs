use crate::domain::{entities::Step, value_objects::ActionDefinition};

/// Request DTO for the
/// [`ResolveActionInputsPort`](crate::application::ports::inbound::resolve_action_inputs_port::ResolveActionInputsPort)
/// inbound port.
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
