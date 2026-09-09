use serde::Serialize;

use super::{repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `repository_dispatch` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RepositoryDispatchPayload {
    action: String,
    client_payload: serde_json::Value,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl RepositoryDispatchPayload {
    pub fn new(
        action: String,
        client_payload: serde_json::Value,
        repository: RepositoryInfo,
        sender: UserInfo,
    ) -> Self {
        Self {
            action,
            client_payload,
            repository,
            sender,
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn client_payload(&self) -> &serde_json::Value {
        &self.client_payload
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
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
        let payload = RepositoryDispatchPayload::new(
            "custom".into(),
            serde_json::json!({"key": "value"}),
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
        );

        assert_eq!(payload.action(), "custom");
        assert_eq!(payload.client_payload()["key"], "value");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
