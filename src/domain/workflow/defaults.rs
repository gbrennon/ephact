use serde::Deserialize;

use super::RunDefaults;

/// Default settings for all jobs in a workflow.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Defaults {
    run: Option<RunDefaults>,
}

impl Defaults {
    pub fn new(run: Option<RunDefaults>) -> Self {
        Self { run }
    }

    pub fn run(&self) -> Option<&RunDefaults> {
        self.run.as_ref()
    }
}
