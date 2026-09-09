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
        Self {
            forkee,
            repository,
            sender,
        }
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
        let payload = ForkPayload::new(
            repository(),
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
        );

        assert_eq!(payload.forkee().name(), "repo");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
