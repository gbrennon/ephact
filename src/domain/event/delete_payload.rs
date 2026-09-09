use serde::Serialize;

use super::{repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `delete` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DeletePayload {
    ref_type: String,
    pub r#ref: String,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl DeletePayload {
    pub fn new(ref_type: String, r#ref: String, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { ref_type, r#ref, repository, sender }
    }

    pub fn ref_type(&self) -> &str {
        &self.ref_type
    }

    pub fn r#ref(&self) -> &str {
        &self.r#ref
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
