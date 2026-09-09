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
    pub fn new(
        action: String,
        issue: IssueInfo,
        comment: CommentInfo,
        repository: RepositoryInfo,
        sender: UserInfo,
    ) -> Self {
        Self {
            action,
            issue,
            comment,
            repository,
            sender,
        }
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
        let user = UserInfo::new("name".into(), "email".into(), "login".into());
        let issue = IssueInfo::new(
            1,
            "title".into(),
            None,
            "open".into(),
            user.clone(),
            Vec::new(),
            "url".into(),
        );
        let comment = CommentInfo::new(2, "body".into(), user.clone(), "comment".into());
        let payload =
            IssueCommentPayload::new("created".into(), issue, comment, repository(), user);

        assert_eq!(payload.action(), "created");
        assert_eq!(payload.issue().number(), 1);
        assert_eq!(payload.comment().id(), 2);
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
