use std::collections::HashMap;

use crate::{domain::errors::StepError, dtos::requests::ResolveActionInputsRequest};

/// Resolves the input values for an action invocation.
pub trait ActionInputsResolverPort: Send + Sync {
    fn resolve(
        &self,
        request: ResolveActionInputsRequest,
    ) -> Result<HashMap<String, String>, StepError>;
}
