use std::collections::HashMap;

use ephact_application::ports::outbound::StepTextCodecPort;
use ephact_domain::{entities::Step, errors::StepError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct StepJson {
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "if")]
    condition: Option<String>,
    run: Option<String>,
    shell: Option<String>,
    working_directory: Option<String>,
    uses: Option<String>,
    with: HashMap<String, String>,
    env: HashMap<String, String>,
    continue_on_error: Option<String>,
    timeout_minutes: Option<f64>,
}

impl From<&Step> for StepJson {
    fn from(step: &Step) -> Self {
        Self {
            id: step.id().map(str::to_owned),
            name: step.name().map(str::to_owned),
            condition: step.if_condition().map(str::to_owned),
            run: step.run().map(str::to_owned),
            shell: step.shell().map(str::to_owned),
            working_directory: step.working_directory().map(str::to_owned),
            uses: step.uses().map(str::to_owned),
            with: step.with().clone(),
            env: step.env().clone(),
            continue_on_error: step.continue_on_error().map(str::to_owned),
            timeout_minutes: step.timeout_minutes(),
        }
    }
}

impl From<StepJson> for Step {
    fn from(step: StepJson) -> Self {
        Step::new(step.id, step.name, step.run, step.uses)
            .with_if_condition(step.condition)
            .with_shell(step.shell)
            .with_working_directory(step.working_directory)
            .with_inputs(step.with)
            .with_env(step.env)
            .with_continue_on_error(step.continue_on_error)
            .with_timeout_minutes(step.timeout_minutes)
    }
}

/// Encodes and decodes steps using the JSON wire format.
#[derive(Debug, Default)]
pub struct JsonStepTextCodec;

impl StepTextCodecPort for JsonStepTextCodec {
    fn decode(&self, text: &str) -> Result<Step, StepError> {
        serde_json::from_str::<StepJson>(text)
            .map(Into::into)
            .map_err(|error| StepError::new(format!("failed to decode step: {error}")))
    }

    fn encode(&self, step: &Step) -> Result<String, StepError> {
        serde_json::to_string(&StepJson::from(step))
            .map_err(|error| StepError::new(format!("failed to encode step: {error}")))
    }
}
