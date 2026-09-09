use std::collections::HashMap;

use serde::Serialize;

use super::{repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `workflow_dispatch` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WorkflowDispatchPayload {
    inputs: HashMap<String, String>,
    repository: RepositoryInfo,
    sender: UserInfo,
    workflow: String,
    pub r#ref: String,
}

impl WorkflowDispatchPayload {
    pub fn new(inputs: HashMap<String, String>, repository: RepositoryInfo, sender: UserInfo, workflow: String, r#ref: String) -> Self {
        Self { inputs, repository, sender, workflow, r#ref }
    }

    pub fn inputs(&self) -> &HashMap<String, String> {
        &self.inputs
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }

    pub fn workflow(&self) -> &str {
        &self.workflow
    }

    pub fn r#ref(&self) -> &str {
        &self.r#ref
    }
}
