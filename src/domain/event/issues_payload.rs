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
    pub fn new(
        action: String,
        issue: IssueInfo,
        repository: RepositoryInfo,
        sender: UserInfo,
    ) -> Self {
        Self {
            action,
            issue,
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
        let issue = IssueInfo::new(
            1,
            "title".into(),
            None,
            "open".into(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            Vec::new(),
            "url".into(),
        );
        let payload = IssuesPayload::new(
            "opened".into(),
            issue,
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
        );

        assert_eq!(payload.action(), "opened");
        assert_eq!(payload.issue().number(), 1);
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
