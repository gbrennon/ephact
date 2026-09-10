use serde::Deserialize;

use crate::domain::value_objects::RunStepDefaults;

/// The `defaults.run:` entry as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct RunStepDefaultsYaml {
    shell: Option<String>,

    #[serde(rename = "working-directory")]
    working_directory: Option<String>,
}

impl RunStepDefaultsYaml {
    /// Builds the domain run-step defaults this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> RunStepDefaults {
        RunStepDefaults::new(self.shell, self.working_directory)
    }
}
