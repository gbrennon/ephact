use crate::{
    application::dtos::requests::LoadActionDefinitionRequest,
    domain::{errors::StepError, value_objects::ActionDefinition},
};

/// Loads and parses an action definition from an action metadata file.
pub trait ActionDefinitionLoaderPort: Send + Sync {
    /// Reads and parses the action's definition.
    fn load(&self, request: LoadActionDefinitionRequest) -> Result<ActionDefinition, StepError>;
}
