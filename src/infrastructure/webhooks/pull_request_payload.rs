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
    pub fn new(
        action: String,
        number: u64,
        pull_request: PullRequestInfo,
        repository: RepositoryInfo,
        sender: UserInfo,
    ) -> Self {
        Self {
            action,
            number,
            pull_request,
            repository,
            sender,
        }
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
#[cfg(test)]
mod tests {
    use super::super::BranchRef;
    use super::*;

    fn repository() -> RepositoryInfo {
        RepositoryInfo::new(
            "repo".into(),
            "owner/repo".into(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            false,
            super::super::repository_info::RepositoryLinks::new(
                "html".into(),
                "main".into(),
                "clone".into(),
                "ssh".into(),
            ),
        )
    }

    fn pull_request() -> PullRequestInfo {
        let branch = BranchRef::new("ref".into(), "sha".into(), repository(), "main".into());
        PullRequestInfo::new(
            1,
            "title".into(),
            None,
            super::super::pull_request_info::PullRequestBranches::new(branch.clone(), branch),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            "url".into(),
            super::super::pull_request_info::PullRequestState::new(false, false, None),
        )
    }

    #[test]
    fn new_preserves_fields() {
        let payload = PullRequestPayload::new(
            "opened".into(),
            1,
            pull_request(),
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
        );

        assert_eq!(payload.action(), "opened");
        assert_eq!(payload.number(), 1);
        assert_eq!(payload.pull_request().title(), "title");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
