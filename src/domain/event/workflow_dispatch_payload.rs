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
    pub fn new(
        inputs: HashMap<String, String>,
        repository: RepositoryInfo,
        sender: UserInfo,
        workflow: String,
        r#ref: String,
    ) -> Self {
        Self {
            inputs,
            repository,
            sender,
            workflow,
            r#ref,
        }
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
#[cfg(test)]
mod tests {
    use super::*;

    fn repository() -> RepositoryInfo {
        RepositoryInfo::new(
            "repo".into(),
            "owner/repo".into(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            false,
            "html".into(),
            "main".into(),
            "clone".into(),
            "ssh".into(),
        )
    }

    #[test]
    fn new_preserves_fields() {
        let payload = WorkflowDispatchPayload::new(
            HashMap::from([("input".into(), "value".into())]),
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            "workflow.yml".into(),
            "main".into(),
        );

        assert_eq!(payload.inputs()["input"], "value");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
        assert_eq!(payload.workflow(), "workflow.yml");
        assert_eq!(payload.r#ref(), "main");
    }
}
