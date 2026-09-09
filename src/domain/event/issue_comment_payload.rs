use serde::Serialize;

use super::{
    comment_info::CommentInfo, issue_info::IssueInfo, repository_info::RepositoryInfo,
    user_info::UserInfo,
};

/// Payload for `issue_comment` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct IssueCommentPayload {
    action: String,
    issue: IssueInfo,
    comment: CommentInfo,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl IssueCommentPayload {
    pub fn new(action: String, issue: IssueInfo, comment: CommentInfo, repository: RepositoryInfo, sender: UserInfo) -> Self {
        Self { action, issue, comment, repository, sender }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn issue(&self) -> &IssueInfo {
        &self.issue
    }

    pub fn comment(&self) -> &CommentInfo {
        &self.comment
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }
}
