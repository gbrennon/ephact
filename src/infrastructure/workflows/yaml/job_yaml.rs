use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::entities::Job,
    infrastructure::workflows::yaml::{
        ConcurrencyGroupYaml, ContainerSpecificationYaml, JobStrategyYaml, StepYaml,
        TokenPermissionsYaml, context_value_from_yaml, job_needs_from_yaml,
    },
};

/// Job entry of a workflow file as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct JobYaml {
    name: Option<String>,

    #[serde(rename = "runs-on")]
    runs_on: Option<String>,

    #[serde(default)]
    steps: Vec<StepYaml>,

    #[serde(default, deserialize_with = "job_needs_from_yaml")]
    needs: Vec<String>,

    #[serde(rename = "if")]
    r#if: Option<String>,

    #[serde(default)]
    strategy: Option<JobStrategyYaml>,

    #[serde(default)]
    env: HashMap<String, String>,

    #[serde(default)]
    container: Option<ContainerSpecificationYaml>,

    #[serde(default)]
    services: HashMap<String, ContainerSpecificationYaml>,

    #[serde(default)]
    outputs: HashMap<String, String>,

    #[serde(default)]
    with: Option<serde_yaml::Value>,

    #[serde(default)]
    secrets: Option<serde_yaml::Value>,

    #[serde(rename = "timeout-minutes")]
    timeout_minutes: Option<f64>,

    #[serde(rename = "continue-on-error")]
    continue_on_error: Option<String>,

    #[serde(default)]
    permissions: Option<TokenPermissionsYaml>,

    #[serde(default)]
    concurrency: Option<ConcurrencyGroupYaml>,
}

impl JobYaml {
    /// Builds the domain job this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> Job {
        let steps = self.steps.into_iter().map(StepYaml::into_domain).collect();
        let services = self
            .services
            .into_iter()
            .map(|(name, service)| (name, service.into_domain()))
            .collect();
        Job::new(
            self.name,
            self.runs_on,
            steps,
            self.needs,
            self.r#if,
            self.strategy.map(JobStrategyYaml::into_domain),
            self.env,
            self.container.map(ContainerSpecificationYaml::into_domain),
            services,
            self.outputs,
            self.with.map(context_value_from_yaml),
            self.secrets.map(context_value_from_yaml),
            self.timeout_minutes,
            self.continue_on_error,
            self.permissions.map(TokenPermissionsYaml::into_domain),
            self.concurrency.map(ConcurrencyGroupYaml::into_domain),
        )
    }
}
