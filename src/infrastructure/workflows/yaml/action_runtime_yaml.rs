use serde::Deserialize;

use crate::{domain::value_objects::ActionRuntime, infrastructure::workflows::yaml::StepYaml};

/// The `runs:` entry of an action definition as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "using")]
pub enum ActionRuntimeYaml {
    /// Composite action running its own steps.
    #[serde(rename = "composite")]
    Composite {
        /// Steps the composite action runs in order.
        steps: Vec<StepYaml>,
    },

    /// JavaScript action executed with Node 12.
    #[serde(rename = "node12")]
    Node12 {
        /// Entry point script.
        main: String,
    },

    /// JavaScript action executed with Node 16.
    #[serde(rename = "node16")]
    Node16 {
        /// Entry point script.
        main: String,
    },

    /// JavaScript action executed with Node 20.
    #[serde(rename = "node20")]
    Node20 {
        /// Entry point script.
        main: String,
    },

    /// Container action executed from an image.
    #[serde(rename = "docker")]
    Docker {
        /// Image the action runs in.
        image: String,
    },
}

impl ActionRuntimeYaml {
    /// Builds the domain action runtime this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ActionRuntime {
        match self {
            Self::Composite { steps } => ActionRuntime::Composite {
                steps: steps.into_iter().map(StepYaml::into_domain).collect(),
            },
            Self::Node12 { main } => ActionRuntime::Node12 { main },
            Self::Node16 { main } => ActionRuntime::Node16 { main },
            Self::Node20 { main } => ActionRuntime::Node20 { main },
            Self::Docker { image } => ActionRuntime::Docker { image },
        }
    }
}
