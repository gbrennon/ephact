use std::collections::HashMap;

use crate::value_objects::ContextValue;

#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobContract {
    /// Outputs produced by this job (for dependent jobs).
    pub(super) outputs: HashMap<String, String>,

    /// Input parameters passed via `workflow_call`.
    pub(super) with: Option<ContextValue>,

    /// Secrets available to this job.
    pub(super) secrets: Option<ContextValue>,
}
