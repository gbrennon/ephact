use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::value_objects::ActionDefinition;

/// Groups the resolved action data required to execute an action.
#[derive(Debug)]
pub struct ActionExecutionContext {
    action_directory: PathBuf,
    definition: ActionDefinition,
    inputs: HashMap<String, String>,
}

impl ActionExecutionContext {
    pub fn new(
        action_directory: PathBuf,
        definition: ActionDefinition,
        inputs: HashMap<String, String>,
    ) -> Self {
        Self {
            action_directory,
            definition,
            inputs,
        }
    }

    pub fn action_directory(&self) -> &Path {
        &self.action_directory
    }

    pub fn definition(&self) -> &ActionDefinition {
        &self.definition
    }

    pub fn inputs(&self) -> &HashMap<String, String> {
        &self.inputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::ActionRuntime;

    fn sample_definition() -> ActionDefinition {
        ActionDefinition::new(
            "Checkout",
            None,
            HashMap::new(),
            ActionRuntime::Node20 {
                main: "index.js".into(),
            },
        )
    }

    fn sample_context(inputs: HashMap<String, String>) -> ActionExecutionContext {
        ActionExecutionContext::new(
            PathBuf::from("/actions/checkout"),
            sample_definition(),
            inputs,
        )
    }

    #[test]
    fn constructor() {
        let directory = PathBuf::from("/actions/checkout");
        let definition = sample_definition();
        let inputs = HashMap::from([("ref".to_string(), "main".to_string())]);

        let context =
            ActionExecutionContext::new(directory.clone(), definition.clone(), inputs.clone());

        assert_eq!(context.action_directory(), directory.as_path());
        assert_eq!(context.definition(), &definition);
        assert_eq!(context.inputs(), &inputs);
    }

    #[test]
    fn action_directory() {
        let context = sample_context(HashMap::new());

        assert_eq!(context.action_directory(), Path::new("/actions/checkout"));
    }

    #[test]
    fn definition() {
        let context = sample_context(HashMap::new());

        assert_eq!(context.definition(), &sample_definition());
    }

    #[test]
    fn inputs() {
        let inputs = HashMap::from([("ref".to_string(), "main".to_string())]);
        let context = sample_context(inputs.clone());

        assert_eq!(context.inputs(), &inputs);
    }
}
