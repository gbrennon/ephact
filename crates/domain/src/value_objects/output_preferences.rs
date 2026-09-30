#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutputPreferences {
    preserve: bool,
    verbose: bool,
}

impl OutputPreferences {
    pub fn preserve(&self) -> bool {
        self.preserve
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
    pub fn with_preserve(mut self, value: bool) -> Self {
        self.preserve = value;
        self
    }
    pub fn with_verbose(mut self, value: bool) -> Self {
        self.verbose = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_disables_every_flag() {
        let preferences = OutputPreferences::default();

        assert!(!preferences.preserve());
        assert!(!preferences.verbose());
    }

    #[test]
    fn with_preserve_enables_preserve() {
        let preferences = OutputPreferences::default().with_preserve(true);

        assert!(preferences.preserve());
        assert!(!preferences.verbose());
    }

    #[test]
    fn with_verbose_enables_verbose() {
        let preferences = OutputPreferences::default().with_verbose(true);

        assert!(preferences.verbose());
        assert!(!preferences.preserve());
    }
}
