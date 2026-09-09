use serde::Serialize;

use super::{issue_info::IssueInfo, repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `issues` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct IssuesPayload {
    action: String,
    issue: IssueInfo,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl IssuesPayload {
    pub fn new(action: String, issue: IssueInfo, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { action, issue, repository, sender }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn issue(&self) -> &IssueInfo {
        &self.issue
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
