use std::collections::HashMap;

use serde::{Deserialize, de::Deserializer};

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

    #[serde(default, deserialize_with = "StepYaml::deserialize_scalar_string_map")]
    with: HashMap<String, String>,

    #[serde(default, deserialize_with = "StepYaml::deserialize_scalar_string_map")]
    env: HashMap<String, String>,

    #[serde(
        rename = "continue-on-error",
        default,
        deserialize_with = "StepYaml::deserialize_optional_scalar_string"
    )]
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

    fn deserialize_scalar_string_map<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<String, String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = HashMap::<String, serde_yaml::Value>::deserialize(deserializer)?;
        raw.into_iter()
            .map(|(key, value)| {
                Ok((
                    key,
                    Self::scalar_to_string(&value).map_err(serde::de::Error::custom)?,
                ))
            })
            .collect()
    }

    fn deserialize_optional_scalar_string<'de, D>(
        deserializer: D,
    ) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<serde_yaml::Value>::deserialize(deserializer)?;
        match value {
            None | Some(serde_yaml::Value::Null) => Ok(None),
            Some(scalar) => Self::scalar_to_string(&scalar)
                .map(Some)
                .map_err(serde::de::Error::custom),
        }
    }

    fn scalar_to_string(value: &serde_yaml::Value) -> Result<String, String> {
        match value {
            serde_yaml::Value::String(text) => Ok(text.clone()),
            serde_yaml::Value::Bool(flag) => Ok(flag.to_string()),
            serde_yaml::Value::Number(number) => Ok(number.to_string()),
            serde_yaml::Value::Null => Ok(String::new()),
            serde_yaml::Value::Sequence(_)
            | serde_yaml::Value::Mapping(_)
            | serde_yaml::Value::Tagged(_) => {
                Err("expected a scalar value (string, number, or boolean)".to_owned())
            }
        }
    }
}
