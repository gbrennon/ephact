use crate::domain::value_objects::{ActionInput, ActionRuntime};

/// A parsed action definition (`action.yml` / `action.yaml`).
///
/// Supports composite actions (`using: composite`) with nested steps.
/// Node and Docker actions are parsed but not yet executed.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact::domain::value_objects::{ActionDefinition, ActionRuntime};
///
/// let action = ActionDefinition::new(
///     "My Action",
///     None,
///     HashMap::new(),
///     ActionRuntime::Composite { steps: Vec::new() },
/// );
///
/// assert_eq!(action.name(), "My Action");
/// assert!(matches!(action.runs(), ActionRuntime::Composite { .. }));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ActionDefinition {
    name: String,

    /// Optional description of what the action does.
    description: Option<String>,

    /// Input parameters declared by the action.
    inputs: std::collections::HashMap<String, ActionInput>,

    /// How the action executes.
    runs: ActionRuntime,
}

impl ActionDefinition {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        inputs: std::collections::HashMap<String, ActionInput>,
        runs: ActionRuntime,
    ) -> Self {
        Self {
            name: name.into(),
            description,
            inputs,
            runs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn inputs(&self) -> &std::collections::HashMap<String, ActionInput> {
        &self.inputs
    }

    pub fn runs(&self) -> &ActionRuntime {
        &self.runs
    }
    /// Returns the input declaration named `name`.
    pub fn input_named(&self, name: &str) -> Option<&ActionInput> {
        self.inputs.get(name)
    }

    /// Returns whether this action declares at least one required input.
    pub fn has_required_inputs(&self) -> bool {
        self.inputs.values().any(ActionInput::required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_preserves_fields() {
        let action = ActionDefinition::new(
            "Action",
            Some("Description".into()),
            std::collections::HashMap::from([("input".into(), ActionInput::new(None, true, None))]),
            ActionRuntime::Node20 {
                main: "index.js".into(),
            },
        );

        assert_eq!(action.name(), "Action");
        assert_eq!(action.description(), Some("Description"));
        assert!(action.inputs()["input"].required());
        assert!(matches!(action.runs(), ActionRuntime::Node20 { .. }));
    }
    #[test]
    fn finds_named_input_and_reports_required_inputs() {
        let action = ActionDefinition::new(
            "Action",
            None,
            std::collections::HashMap::from([
                ("required".into(), ActionInput::new(None, true, None)),
                ("optional".into(), ActionInput::new(None, false, None)),
            ]),
            ActionRuntime::Node20 {
                main: "index.js".into(),
            },
        );

        assert!(action.input_named("required").is_some());
        assert!(action.input_named("missing").is_none());
        assert!(action.has_required_inputs());
    }
}
