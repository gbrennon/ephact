use crate::application::dtos::requests::LoadActionDefinitionRequest;
use crate::domain::errors::StepError;
use crate::domain::value_objects::ActionDefinition;

/// Inbound port for reading an action's `action.yml`.
pub trait LoadActionDefinitionPort: Send + Sync {
    /// Reads and parses the action's definition.
    fn execute(
        &self,
        request: LoadActionDefinitionRequest<'_>,
    ) -> Result<ActionDefinition, StepError>;
}
