use std::collections::HashMap;

use crate::{application::dtos::requests::ResolveActionInputsRequest, domain::errors::StepError};

/// Resolves the input values for an action invocation.
pub trait ActionInputsResolverPort: Send + Sync {
    fn resolve(
        &self,
        request: ResolveActionInputsRequest,
    ) -> Result<HashMap<String, String>, StepError>;
}
