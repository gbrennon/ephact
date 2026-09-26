use std::collections::HashMap;

use serde::Deserialize;

use crate::domain::entities::Step;

/// Step entry of a job or composite action as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct StepYaml {
    id: Option<String>,

    name: Option<String>,

    #[serde(rename = "if")]
    r#if: Option<String>,

    #[serde(default)]
    run: Option<String>,

    #[serde(default)]
    shell: Option<String>,

    #[serde(rename = "working-directory")]
    working_directory: Option<String>,

    #[serde(default)]
    uses: Option<String>,

    #[serde(default)]
    with: HashMap<String, String>,

    #[serde(default)]
    env: HashMap<String, String>,

    #[serde(rename = "continue-on-error")]
    continue_on_error: Option<String>,

    #[serde(rename = "timeout-minutes")]
    timeout_minutes: Option<f64>,
}

impl StepYaml {
    /// Builds the domain step this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> Step {
        Step::new(self.id, self.name, self.run, self.uses)
            .with_if_condition(self.r#if)
            .with_shell(self.shell)
            .with_working_directory(self.working_directory)
            .with_inputs(self.with)
            .with_env(self.env)
            .with_continue_on_error(self.continue_on_error)
            .with_timeout_minutes(self.timeout_minutes)
    }
}
