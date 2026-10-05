use crate::{
    dtos::{requests::ListActionsRequest, responses::ListActionsResponse},
    errors::ListActionsError,
};

/// Inbound port for listing actions referenced across workflows.
pub trait ListActionsPort {
    /// Discovers and lists all actions used in the repository's workflows.
    /// The response contains the discovered action references.
    ///
    /// # Errors
    ///
    /// Returns [`ListActionsError`] when the repository cannot be resolved or
    /// its workflow source cannot be read.
    fn execute(&self, request: ListActionsRequest)
    -> Result<ListActionsResponse, ListActionsError>;
}
