use serde::Deserialize;

use super::{ActionInput, ActionRuns};

/// A parsed GitHub Actions action definition (`action.yml` / `action.yaml`).
///
/// Supports composite actions (`using: composite`) with nested steps.
/// Node and Docker actions are parsed but not yet executed.
///
/// # Examples
///
/// ```
/// use ephact::domain::workflow::{ActionDefinition, ActionRuns};
///
/// let yaml = r#"
/// name: My Action
/// runs:
///   using: composite
///   steps:
///     - run: echo hello
///       shell: bash
/// "#;
/// let action: ActionDefinition = serde_yaml::from_str(yaml).unwrap();
/// assert_eq!(action.name(), "My Action");
/// assert!(matches!(action.runs(), ActionRuns::Composite { .. }));
/// ```
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ActionDefinition {
    name: String,

    /// Optional description of what the action does.
    #[serde(default)]
    description: Option<String>,

    /// Input parameters declared by the action.
    #[serde(default)]
    inputs: std::collections::HashMap<String, ActionInput>,

    /// How the action executes.
    runs: ActionRuns,
}

impl ActionDefinition {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        inputs: std::collections::HashMap<String, ActionInput>,
        runs: ActionRuns,
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

    pub fn runs(&self) -> &ActionRuns {
        &self.runs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_composite_action() {
        let yaml = r#"
name: Test Action
description: Does stuff
runs:
  using: composite
  steps:
    - run: echo hello
      shell: bash
    - name: Check
      run: cargo test
"#;
        let action: ActionDefinition = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(action.name(), "Test Action");
        assert_eq!(action.description(), Some("Does stuff"));
        assert!(matches!(action.runs(), ActionRuns::Composite { .. }));
        if let ActionRuns::Composite { steps } = action.runs() {
            assert_eq!(steps.len(), 2);
            assert_eq!(steps[0].run(), Some("echo hello"));
            assert_eq!(steps[1].name(), Some("Check"));
        }
    }

    #[test]
    fn parses_action_with_inputs() {
        let yaml = r#"
name: With Inputs
inputs:
  path:
    description: Files to cache
    required: true
  key:
    description: Cache key
    default: default-key
runs:
  using: composite
  steps:
    - run: echo done
"#;
        let action: ActionDefinition = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(action.inputs().len(), 2);
        assert!(action.inputs()["path"].required());
        assert_eq!(action.inputs()["key"].default(), Some("default-key"));
    }

    #[test]
    fn parses_node16_action() {
        let yaml = r#"
name: Node Action
runs:
  using: node16
  main: index.js
"#;
        let action: ActionDefinition = serde_yaml::from_str(yaml).unwrap();
        assert!(matches!(action.runs(), ActionRuns::Node16 { .. }));
    }

    #[test]
    fn parses_docker_action() {
        let yaml = r#"
name: Docker Action
runs:
  using: docker
  image: Dockerfile
"#;
        let action: ActionDefinition = serde_yaml::from_str(yaml).unwrap();
        assert!(matches!(action.runs(), ActionRuns::Docker { .. }));
    }
}
