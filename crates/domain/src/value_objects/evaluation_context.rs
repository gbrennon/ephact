use std::collections::BTreeMap;

use crate::value_objects::ContextValue;

/// Values available to expression evaluation.
///
/// Root names are supplied by the workflow dialect adapter. The domain keeps
/// the context generic so it does not depend on a particular forge or runner.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EvaluationContext {
    roots: BTreeMap<String, ContextValue>,
}

impl EvaluationContext {
    /// Creates an empty evaluation context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Looks up a context root by its caller-defined name.
    pub fn get(&self, name: &str) -> Option<&ContextValue> {
        self.roots.get(name)
    }

    /// Returns all context roots.
    pub fn roots(&self) -> &BTreeMap<String, ContextValue> {
        &self.roots
    }

    /// Adds or replaces a context root.
    pub fn with_root(mut self, name: impl Into<String>, value: ContextValue) -> Self {
        self.roots.insert(name.into(), value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_an_empty_context() {
        assert!(EvaluationContext::new().roots().is_empty());
    }

    #[test]
    fn get_known_root() {
        let context = EvaluationContext::new().with_root("source", ContextValue::text("value"));

        assert_eq!(context.get("source"), Some(&ContextValue::text("value")));
    }

    #[test]
    fn get_unknown_root() {
        assert!(EvaluationContext::new().get("missing").is_none());
    }

    #[test]
    fn default_equals_new() {
        assert_eq!(EvaluationContext::new(), EvaluationContext::default());
    }

    #[test]
    fn with_root_replaces_a_value() {
        let context = EvaluationContext::new()
            .with_root("source", ContextValue::text("first"))
            .with_root("source", ContextValue::text("second"));

        assert_eq!(context.get("source"), Some(&ContextValue::text("second")));
    }
}
