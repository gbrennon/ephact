use crate::{
    domain::{aggregates::Workflow, value_objects::TriggerKind},
    workflows::{actions::WorkflowYaml, woodpecker::WoodpeckerPipelineYaml},
};

/// A format-dispatched workflow document.
///
/// Centralizes format selection and domain conversion for the two supported
/// workflow formats, so every consumer parses exactly once.
pub enum WorkflowDocument {
    /// A GitHub Actions / Forgejo Actions workflow.
    Actions(Box<WorkflowYaml>),
    /// A Woodpecker CI pipeline.
    Woodpecker(WoodpeckerPipelineYaml),
}

impl WorkflowDocument {
    /// Parses YAML content and selects the matching format variant.
    ///
    /// Woodpecker is detected first by checking for `when:` + `steps:` keys.
    /// Everything else is treated as an Actions workflow.
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        if WoodpeckerPipelineYaml::is_document(content) {
            return WoodpeckerPipelineYaml::parse(content).map(Self::Woodpecker);
        }
        serde_yaml::from_str::<WorkflowYaml>(content).map(|y| Self::Actions(Box::new(y)))
    }

    /// Returns the top-level workflow name, if declared.
    ///
    /// Never returns a nested step or job name.
    pub fn name(&self) -> Option<String> {
        self.to_domain().name().map(str::to_owned)
    }

    /// Returns the declared event names for this workflow.
    ///
    /// Event names match the canonical Actions strings:
    /// `"push"`, `"pull_request"`, `"workflow_dispatch"`, or `"schedule"`.
    pub fn trigger_names(&self) -> Vec<String> {
        self.to_domain()
            .trigger()
            .iter()
            .map(|trigger| match trigger.kind() {
                TriggerKind::Push => "push",
                TriggerKind::PullRequest => "pull_request",
                TriggerKind::Manual => "workflow_dispatch",
                TriggerKind::Schedule => "schedule",
            })
            .map(str::to_owned)
            .collect()
    }

    /// Converts this document into its domain workflow.
    pub fn into_domain(self) -> Workflow {
        match self {
            Self::Actions(yaml) => yaml.into_domain(),
            Self::Woodpecker(pipeline) => pipeline.into_domain(),
        }
    }

    /// Converts to domain by cloning the inner YAML representation.
    ///
    /// Used by `name` and `trigger_names` which borrow `self`.
    fn to_domain(&self) -> Workflow {
        match self {
            Self::Actions(yaml) => yaml.clone().into_domain(),
            Self::Woodpecker(pipeline) => pipeline.clone().into_domain(),
        }
    }
}
