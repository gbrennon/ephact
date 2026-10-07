use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::{aggregates::Workflow, value_objects::ContextValue},
    workflows::actions::{
        ConcurrencyGroupYaml, ExecutionDefaultsYaml, JobYaml, TokenPermissionsYaml,
        WorkflowTriggerYaml,
    },
};

/// Workflow file as authored in YAML.
///
/// Carries the on-disk field names and defaults, and maps onto the
/// [`Workflow`] aggregate once parsed.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowYaml {
    name: Option<String>,

    #[serde(default)]
    #[serde(rename = "on")]
    trigger: WorkflowTriggerYaml,

    #[serde(default)]
    env: HashMap<String, String>,

    #[serde(default)]
    jobs: HashMap<String, JobYaml>,

    #[serde(default)]
    defaults: Option<ExecutionDefaultsYaml>,

    #[serde(default)]
    permissions: Option<TokenPermissionsYaml>,

    #[serde(default)]
    concurrency: Option<ConcurrencyGroupYaml>,
}

impl WorkflowYaml {
    /// Builds the domain workflow this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> Workflow {
        let name = self
            .name
            .map(|name| name.trim().to_owned())
            .filter(|name| !name.is_empty());
        let jobs = self
            .jobs
            .into_iter()
            .map(|(id, job)| (id, job.into_domain()))
            .collect();
        Workflow::new(name, self.trigger.into_domain(), self.env, jobs)
            .with_defaults(self.defaults.map(ExecutionDefaultsYaml::into_domain))
            .with_permissions(self.permissions.map(TokenPermissionsYaml::into_domain))
            .with_concurrency(self.concurrency.map(ConcurrencyGroupYaml::into_domain))
    }

    /// Converts a YAML value into the context value used by workflow expressions.
    pub fn context_value_from_yaml(value: serde_yaml::Value) -> ContextValue {
        match value {
            serde_yaml::Value::Null => ContextValue::Null,
            serde_yaml::Value::Bool(flag) => ContextValue::Boolean(flag),
            serde_yaml::Value::Number(number) => Self::number_from_yaml(&number),
            serde_yaml::Value::String(text) => ContextValue::Text(text),
            serde_yaml::Value::Sequence(items) => {
                ContextValue::list(items.into_iter().map(Self::context_value_from_yaml))
            }
            serde_yaml::Value::Mapping(entries) => Self::mapping_from_yaml(entries),
            serde_yaml::Value::Tagged(tagged) => Self::context_value_from_yaml(tagged.value),
        }
    }

    fn number_from_yaml(number: &serde_yaml::Number) -> ContextValue {
        match number.as_i64() {
            Some(integer) => ContextValue::Integer(integer),
            None => ContextValue::Decimal(number.as_f64().unwrap_or_default()),
        }
    }

    fn mapping_from_yaml(entries: serde_yaml::Mapping) -> ContextValue {
        ContextValue::mapping(
            entries
                .into_iter()
                .map(|(key, value)| (Self::key_text(key), Self::context_value_from_yaml(value))),
        )
    }

    fn key_text(key: serde_yaml::Value) -> String {
        match key {
            serde_yaml::Value::String(text) => text,
            other => serde_yaml::to_string(&other)
                .unwrap_or_default()
                .trim_end()
                .to_owned(),
        }
    }
}
