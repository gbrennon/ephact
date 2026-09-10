/// Default run settings for shell and working directory.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RunStepDefaults {
    shell: Option<String>,
    working_directory: Option<String>,
}

impl RunStepDefaults {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_fields() {
        let defaults = RunStepDefaults::new(Some("bash".into()), Some("./src".into()));

        assert_eq!(defaults.shell(), Some("bash"));
        assert_eq!(defaults.working_directory(), Some("./src"));
    }
}
