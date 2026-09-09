use serde::Serialize;

use super::{repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `fork` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ForkPayload {
    forkee: RepositoryInfo,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl ForkPayload {
    pub fn new(forkee: RepositoryInfo, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { forkee, repository, sender }
    }

    pub fn forkee(&self) -> &RepositoryInfo {
        &self.forkee
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
