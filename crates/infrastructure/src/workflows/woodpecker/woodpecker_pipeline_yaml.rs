use std::collections::HashMap;

use serde::Deserialize;

use super::WoodpeckerStepYaml;
use crate::domain::{aggregates::Workflow, entities::Job};

/// A Woodpecker pipeline configuration file.
///
/// Detected by the presence of both `when` and `steps`, each step runs
/// sequentially in its own container sharing the workflow workspace.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerPipelineYaml {
    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    when: Vec<super::WoodpeckerWhenYaml>,

    #[serde(default)]
    steps: serde_yaml::Value,
}

impl WoodpeckerPipelineYaml {
    /// Returns whether the YAML document looks like a Woodpecker pipeline.
    pub fn is_document(content: &str) -> bool {
        let document: serde_yaml::Value = match serde_yaml::from_str(content) {
            Ok(document) => document,
            Err(_) => return false,
        };
        document.get("when").is_some() && document.get("steps").is_some()
    }

    /// Parses the pipeline from YAML content and validates every step.
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        let pipeline: Self = serde_yaml::from_str(content)?;
        pipeline.validate_steps()?;
        Ok(pipeline)
    }

    /// Builds the domain workflow this pipeline describes.
    pub fn into_domain(self) -> Workflow {
        let Self { name, when, steps } = self;
        let triggers = when
            .into_iter()
            .flat_map(super::WoodpeckerWhenYaml::into_triggers)
            .collect();
        let steps = match steps {
            serde_yaml::Value::Sequence(values) => values
                .into_iter()
                .map(|value| {
                    serde_yaml::from_value(value).expect("validated Woodpecker sequence step")
                })
                .collect(),
            serde_yaml::Value::Mapping(values) => values
                .into_iter()
                .map(|(name, value)| {
                    let name = name
                        .as_str()
                        .expect("validated Woodpecker step name")
                        .to_owned();
                    let step: WoodpeckerStepYaml =
                        serde_yaml::from_value(value).expect("validated Woodpecker mapping step");
                    step.with_name_if_absent(name)
                })
                .collect(),
            _ => Vec::new(),
        };
        let jobs = Self::jobs_from_steps(steps);
        Workflow::new(name, triggers, HashMap::new(), jobs)
    }

    fn validate_steps(&self) -> Result<(), serde_yaml::Error> {
        let validate_step = |step: &serde_yaml::Value| {
            let _: WoodpeckerStepYaml = serde_yaml::from_value(step.clone())?;
            Ok(())
        };
        match &self.steps {
            serde_yaml::Value::Sequence(steps) => steps.iter().try_for_each(validate_step),
            serde_yaml::Value::Mapping(steps) => steps.iter().try_for_each(|(name, step)| {
                let _: String = serde_yaml::from_value(name.clone())?;
                validate_step(step)
            }),
            _ => Ok(()),
        }
    }

    fn jobs_from_steps(steps: Vec<WoodpeckerStepYaml>) -> HashMap<String, Job> {
        let ids: Vec<String> = (0..steps.len())
            .map(|index| format!("step-{index}"))
            .collect();
        steps
            .into_iter()
            .enumerate()
            .map(|(index, step)| {
                let needs = index
                    .checked_sub(1)
                    .map(|previous| vec![ids[previous].clone()])
                    .unwrap_or_default();
                (format!("step-{index}"), step.into_domain_job(needs))
            })
            .collect()
    }
}
