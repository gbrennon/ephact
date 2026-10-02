use std::{collections::HashMap, fmt};

use serde::{
    Deserialize,
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
};

use super::{woodpecker_step_yaml::WoodpeckerStepYaml, woodpecker_when_yaml::WoodpeckerWhenYaml};
use crate::domain::{aggregates::Workflow, entities::Job};

/// Woodpecker steps declared either as a sequence or as a name-keyed mapping.
///
/// Woodpecker accepts both forms; the mapping form keys each step by its name
/// while preserving declaration order. Both normalize to an ordered list of
/// steps so the rest of the pipeline conversion is form-agnostic.
#[derive(Debug, Clone, PartialEq, Default)]
struct WoodpeckerSteps(Vec<WoodpeckerStepYaml>);

impl WoodpeckerSteps {
    fn into_vec(self) -> Vec<WoodpeckerStepYaml> {
        self.0
    }
}

impl<'de> Deserialize<'de> for WoodpeckerSteps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StepsVisitor;

        impl<'de> Visitor<'de> for StepsVisitor {
            type Value = Vec<WoodpeckerStepYaml>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence or a name-keyed mapping of Woodpecker steps")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut steps = Vec::new();
                while let Some(step) = seq.next_element::<WoodpeckerStepYaml>()? {
                    steps.push(step);
                }
                Ok(steps)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut steps = Vec::new();
                while let Some((name, step)) = map.next_entry::<String, WoodpeckerStepYaml>()? {
                    steps.push(step.with_name_if_absent(name));
                }
                Ok(steps)
            }
        }

        deserializer
            .deserialize_any(StepsVisitor)
            .map(WoodpeckerSteps)
            .map_err(de::Error::custom)
    }
}

/// A Woodpecker pipeline configuration file.
///
/// Detected by the presence of both `when` and `steps`, each step runs
/// sequentially in its own container sharing the workflow workspace.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerPipelineYaml {
    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    when: Vec<WoodpeckerWhenYaml>,

    #[serde(default)]
    steps: WoodpeckerSteps,
}

impl WoodpeckerPipelineYaml {
    /// Returns whether the YAML document looks like a Woodpecker pipeline.
    pub fn is_document(content: &str) -> bool {
        let Ok(document) = serde_yaml::from_str::<serde_yaml::Value>(content) else {
            return false;
        };
        document.get("when").is_some() && document.get("steps").is_some()
    }

    /// Parses the pipeline from YAML content.
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    /// Builds the domain workflow this pipeline describes.
    pub fn into_domain(self) -> Workflow {
        let triggers = self
            .when
            .into_iter()
            .flat_map(WoodpeckerWhenYaml::into_triggers)
            .collect();
        let jobs = Self::jobs_from_steps(self.steps.into_vec());
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
