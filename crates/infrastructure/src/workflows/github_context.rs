use crate::domain::value_objects::EvaluationContext;

/// The evaluated GitHub context and the original event payload, when present.
#[derive(Debug, Clone)]
pub struct GithubContext {
    context: EvaluationContext,
    event_payload: Option<String>,
}

impl GithubContext {
    /// Creates a context with its source event payload.
    pub fn new(context: EvaluationContext, event_payload: Option<String>) -> Self {
        Self {
            context,
            event_payload,
        }
    }

    /// Returns the evaluated expression context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    /// Returns the original event payload, when an event file was configured.
    pub fn event_payload(&self) -> Option<&str> {
        self.event_payload.as_deref()
    }

    /// Consumes the carrier and returns both values.
    pub fn into_parts(self) -> (EvaluationContext, Option<String>) {
        (self.context, self.event_payload)
    }
}
