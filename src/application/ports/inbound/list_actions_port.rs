use crate::application::dtos::requests::ListActionsRequest;
use crate::application::dtos::responses::ListActionsResponse;
use crate::application::errors::ListActionsError;

/// Inbound port for listing actions referenced across workflows.
pub trait ListActionsPort {
    /// Discovers and lists all actions used in the repository's workflows.
    fn execute(&self, request: ListActionsRequest)
    -> Result<ListActionsResponse, ListActionsError>;
}
