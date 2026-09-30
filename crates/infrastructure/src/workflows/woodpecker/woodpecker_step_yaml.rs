use serde::Deserialize;

use crate::domain::{
    entities::{Job, Step},
    value_objects::ContainerSpecification,
};

/// A single Woodpecker pipeline step.
///
/// Woodpecker runs each step sequentially in its own `image`, executing the
/// listed `commands` as a shell script.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerStepYaml {
    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    image: Option<String>,

    #[serde(default)]
    commands: Vec<String>,
}

impl WoodpeckerStepYaml {
    /// Maps this Woodpecker step into a domain [`Job`] with a single run step.
    ///
    /// The step `image` becomes the job container and `needs` carries the
    /// dependency on the preceding step, preserving Woodpecker's sequential
    /// execution order once mapped onto the parallel-by-default job model.
    #[must_use]
    pub fn into_domain_job(self, needs: Vec<String>) -> Job {
        let script = self.commands.join("\n");
        let step = Step::new(None, self.name.clone(), Some(script), None);
        let container = self.image.map(ContainerSpecification::new);
        Job::new(self.name, None, vec![step], needs).with_container(container)
    }
}
