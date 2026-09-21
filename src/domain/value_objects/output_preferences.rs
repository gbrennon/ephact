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
