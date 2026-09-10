use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::aggregates::Workflow,
    infrastructure::workflows::yaml::{
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
        let jobs = self
            .jobs
            .into_iter()
            .map(|(id, job)| (id, job.into_domain()))
            .collect();
        Workflow::new(
            self.name,
            None,
            self.trigger.into_domain(),
            self.env,
            jobs,
            self.defaults.map(ExecutionDefaultsYaml::into_domain),
            self.permissions.map(TokenPermissionsYaml::into_domain),
            self.concurrency.map(ConcurrencyGroupYaml::into_domain),
        )
    }
}
