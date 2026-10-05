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
    /// Parses one Woodpecker step from YAML content.
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    /// Returns the step's declared name, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the step name when it was not declared inline.
    ///
    /// Woodpecker's mapping form keys each step by name, so the key supplies
    /// the name that the sequence form would carry on the step itself.
    pub fn with_name_if_absent(mut self, name: String) -> Self {
        if self.name.is_none() {
            self.name = Some(name);
        }
        self
    }

    /// Maps this Woodpecker step into a domain [`Job`] with a single run step.
    ///
    /// The step `image` becomes the job container and `needs` carries the
    /// dependency on the preceding step, preserving Woodpecker's sequential
    /// execution order once mapped onto the parallel-by-default job model.
    pub fn into_domain_job(self, needs: Vec<String>) -> Job {
        let script = Self::unescape_dollar_escapes(&self.commands.join("\n"));
        let step = Step::new(None, self.name.clone(), Some(script), None);
        let container = self.image.map(ContainerSpecification::new);
        Job::new(self.name, None, vec![step], needs).with_container(container)
    }

    fn unescape_dollar_escapes(script: &str) -> String {
        script.replace("$$", "$")
    }
}
