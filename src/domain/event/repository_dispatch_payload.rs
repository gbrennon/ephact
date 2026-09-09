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
    pub fn new(action: String, client_payload: serde_json::Value, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { action, client_payload, repository, sender }
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
