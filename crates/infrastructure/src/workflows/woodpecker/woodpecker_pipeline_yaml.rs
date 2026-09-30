use std::collections::HashMap;

use serde::Deserialize;

use super::{woodpecker_step_yaml::WoodpeckerStepYaml, woodpecker_when_yaml::WoodpeckerWhenYaml};
use crate::domain::{aggregates::Workflow, entities::Job};

/// A Woodpecker pipeline file (`.woodpecker/*.yml` or `.woodpecker.yml`).
///
/// This is the Woodpecker-exclusive counterpart to the Actions `WorkflowYaml`:
/// it understands `when:`/`steps:` and maps them onto the shared domain
/// [`Workflow`] aggregate so the rest of the engine can treat it uniformly.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerPipelineYaml {
    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    when: Vec<WoodpeckerWhenYaml>,

    #[serde(default)]
    steps: Vec<WoodpeckerStepYaml>,
}

impl WoodpeckerPipelineYaml {
    /// Returns whether YAML content has the Woodpecker pipeline shape.
    pub fn is_document(content: &str) -> bool {
        let Ok(document) = serde_yaml::from_str::<serde_yaml::Value>(content) else {
            return false;
        };
        document.get("when").is_some() && document.get("steps").is_some()
    }

    /// Parses a Woodpecker pipeline from its YAML source.
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    /// Builds the domain workflow this pipeline describes.
    #[must_use]
    pub fn into_domain(self) -> Workflow {
        let triggers = self
            .when
            .into_iter()
            .flat_map(WoodpeckerWhenYaml::into_triggers)
            .collect();
        let jobs = Self::jobs_from_steps(self.steps);
        Workflow::new(self.name, triggers, HashMap::new(), jobs)
    }

    fn jobs_from_steps(steps: Vec<WoodpeckerStepYaml>) -> HashMap<String, Job> {
        let ids: Vec<String> = (0..steps.len()).map(Self::job_id).collect();
        steps
            .into_iter()
            .enumerate()
            .map(|(index, step)| {
                let needs = index
                    .checked_sub(1)
                    .map(|previous| vec![ids[previous].clone()])
                    .unwrap_or_default();
                (Self::job_id(index), step.into_domain_job(needs))
            })
            .collect()
    }

    fn job_id(index: usize) -> String {
        format!("step-{index}")
    }
}
