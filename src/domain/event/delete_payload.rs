use serde::Serialize;

use super::{repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `delete` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DeletePayload {
    ref_type: String,
    r#ref: String,
    repository: RepositoryInfo,
    sender: UserInfo,
}

impl DeletePayload {
    pub fn new(
        ref_type: String,
        r#ref: String,
        repository: RepositoryInfo,
        sender: UserInfo,
    ) -> Self {
        Self {
            ref_type,
            r#ref,
            repository,
            sender,
        }
    }

    pub fn ref_type(&self) -> &str {
        &self.ref_type
    }

    pub fn r#ref(&self) -> &str {
        &self.r#ref
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
            super::super::repository_info::RepositoryLinks::new(
                "html".into(),
                "main".into(),
                "clone".into(),
                "ssh".into(),
            ),
        )
    }

    #[test]
    fn new_preserves_fields() {
        let payload = DeletePayload::new(
            "branch".into(),
            "main".into(),
            repository(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
        );

        assert_eq!(payload.ref_type(), "branch");
        assert_eq!(payload.r#ref(), "main");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.sender().login(), "login");
    }
}
