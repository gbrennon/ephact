use serde::Serialize;

use super::{repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `create` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CreatePayload {
    ref_type: String,
    pub r#ref: String,
    master_branch: String,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl CreatePayload {
    pub fn new(ref_type: String, r#ref: String, master_branch: String, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { ref_type, r#ref, master_branch, repository, sender }
    }

    pub fn ref_type(&self) -> &str {
        &self.ref_type
    }

    pub fn r#ref(&self) -> &str {
        &self.r#ref
    }

    pub fn master_branch(&self) -> &str {
        &self.master_branch
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
