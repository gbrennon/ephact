use serde::Deserialize;

/// Default run settings for shell and working directory.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct RunDefaults {
    shell: Option<String>,

    #[serde(rename = "working-directory")]
    working_directory: Option<String>,
}

impl RunDefaults {
    pub fn new(shell: Option<String>, working_directory: Option<String>) -> Self {
        Self {
            shell,
            working_directory,
        }
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }
}
