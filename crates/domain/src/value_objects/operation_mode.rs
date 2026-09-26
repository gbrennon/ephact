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
