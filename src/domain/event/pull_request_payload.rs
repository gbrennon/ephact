use serde::Serialize;

use super::{
    pull_request_info::PullRequestInfo, repository_info::RepositoryInfo, user_info::UserInfo,
};

/// Payload for `pull_request` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PullRequestPayload {
    action: String,
    number: u64,
    pull_request: PullRequestInfo,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl PullRequestPayload {
    pub fn new(action: String, number: u64, pull_request: PullRequestInfo, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { action, number, pull_request, repository, sender }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn pull_request(&self) -> &PullRequestInfo {
        &self.pull_request
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
