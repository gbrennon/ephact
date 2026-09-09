use serde::Serialize;

use super::{release_info::ReleaseInfo, repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `release` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReleasePayload {
    action: String,
    release: ReleaseInfo,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl ReleasePayload {
    pub fn new(action: String, release: ReleaseInfo, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { action, release, repository, sender }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn release(&self) -> &ReleaseInfo {
        &self.release
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
