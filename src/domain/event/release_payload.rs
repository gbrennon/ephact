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
    pub fn new(
        action: String,
        release: ReleaseInfo,
        repository: RepositoryInfo,
        sender: UserInfo,
    ) -> Self {
        Self {
            action,
            release,
            repository,
            sender,
        }
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
        let release = ReleaseInfo::new("v1".into(), None, None, false, false, "url".into());
        let payload = ReleasePayload::new(
            "published".into(),
            release,
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
        );

        assert_eq!(payload.action(), "published");
        assert_eq!(payload.release().tag_name(), "v1");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
