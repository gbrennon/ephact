use std::{collections::HashMap, path::PathBuf};

use super::run_action_execution_input::RunActionExecutionInput;
use crate::{
    domain::{
        entities::Step,
        errors::StepError,
        value_objects::{ContextValue, EvaluationContext},
    },
    ports::outbound::StepTextCodecPort,
};

/// Input data for constructing a [`super::run_action_request::RunActionRequest`].
pub struct RunActionRequestInput {
    action_ref: String,
    step: String,
    execution: RunActionExecutionInput,
}

impl RunActionRequestInput {
    /// Creates input from an action reference, step name, and execution data.
    pub fn new(action_ref: String, step: String, execution: RunActionExecutionInput) -> Self {
        Self {
            action_ref,
            step,
            execution,
        }
    }

    /// Creates input for executing one action without selecting a workflow.
    pub fn for_action(
        action_ref: String,
        inputs: HashMap<String, String>,
        repository_path: PathBuf,
        step_codec: &dyn StepTextCodecPort,
    ) -> Result<Self, StepError> {
        let step =
            Step::new(None, None, None, Some(action_ref.clone())).with_inputs(inputs.clone());
        let step = step_codec.encode(&step)?;
        let context = EvaluationContext::new().with_root(
            "inputs",
            ContextValue::mapping(
                inputs
                    .iter()
                    .map(|(name, value)| (name.clone(), ContextValue::text(value.clone()))),
            ),
        );

        Ok(Self::new(
            action_ref,
            step,
            RunActionExecutionInput::new(repository_path, HashMap::new(), context),
        ))
    }

    /// Consumes the input and returns its action reference, step, and execution data.
    pub fn into_parts(self) -> (String, String, RunActionExecutionInput) {
        (self.action_ref, self.step, self.execution)
    }
}
