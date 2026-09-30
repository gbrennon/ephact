#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OperationMode {
    interactive: bool,
    all_workflows: bool,
}

impl OperationMode {
    pub fn interactive(&self) -> bool {
        self.interactive
    }
    pub fn all_workflows(&self) -> bool {
        self.all_workflows
    }
    pub fn with_interactive(mut self, value: bool) -> Self {
        self.interactive = value;
        self
    }
    pub fn with_all_workflows(mut self, value: bool) -> Self {
        self.all_workflows = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_disables_every_flag() {
        let mode = OperationMode::default();

        assert!(!mode.interactive());
        assert!(!mode.all_workflows());
    }

    #[test]
    fn with_interactive_enables_interactive() {
        let mode = OperationMode::default().with_interactive(true);

        assert!(mode.interactive());
        assert!(!mode.all_workflows());
    }

    #[test]
    fn with_all_workflows_enables_all_workflows() {
        let mode = OperationMode::default().with_all_workflows(true);

        assert!(mode.all_workflows());
        assert!(!mode.interactive());
    }
}
