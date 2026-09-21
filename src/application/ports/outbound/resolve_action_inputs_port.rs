use std::collections::HashMap;

use crate::{application::dtos::requests::ResolveActionInputsRequest, domain::errors::StepError};

/// Inbound port for resolving the inputs an action runs with.
pub trait ResolveActionInputsPort: Send + Sync {
    fn execute(
        &self,
        request: ResolveActionInputsRequest,
    ) -> Result<HashMap<String, String>, StepError>;
}
