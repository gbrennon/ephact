use serde::Deserialize;

/// Concurrency configuration to limit parallel workflow runs.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Concurrency {
    group: String,

    #[serde(rename = "cancel-in-progress")]
    cancel_in_progress: Option<bool>,
}

impl Concurrency {
    pub fn new(group: impl Into<String>, cancel_in_progress: Option<bool>) -> Self {
        Self {
            group: group.into(),
            cancel_in_progress,
        }
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn cancel_in_progress(&self) -> Option<bool> {
        self.cancel_in_progress
    }
}
