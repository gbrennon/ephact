use crate::domain::workflow::{ActionDefinition, Step};

/// Request DTO for the
/// [`ResolveActionInputsPort`](crate::application::ports::inbound::resolve_action_inputs_port::ResolveActionInputsPort)
/// inbound port.
pub struct ResolveActionInputsRequest<'a> {
    /// Definition declaring the action's inputs and their defaults.
    definition: &'a ActionDefinition,
    /// Step that referenced the action, for its `with:` values.
    step: &'a Step,
}

impl<'a> ResolveActionInputsRequest<'a> {
    /// Creates a new request.
    pub fn new(definition: &'a ActionDefinition, step: &'a Step) -> Self {
        Self { definition, step }
    }

    /// Definition declaring the action's inputs and their defaults.
    pub fn definition(&self) -> &'a ActionDefinition {
        self.definition
    }

    /// Step that referenced the action, for its `with:` values.
    pub fn step(&self) -> &'a Step {
        self.step
    }
}
