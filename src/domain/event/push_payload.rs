use serde::Serialize;

use super::{commit_info::CommitInfo, repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `push` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PushPayload {
    r#ref: String,
    before: String,
    after: String,
    repository: RepositoryInfo,
    pusher: UserInfo,
    sender: UserInfo,
    created: bool,
    deleted: bool,
    forced: bool,
    commits: Vec<CommitInfo>,
    head_commit: Option<CommitInfo>,
    compare: String,
}

/// Groups the repository, actors, and state flags of a push event.
pub struct PushMetadata {
    repository: RepositoryInfo,
    pusher: UserInfo,
    sender: UserInfo,
    created: bool,
    deleted: bool,
    forced: bool,
}

impl PushMetadata {
    /// Creates the metadata associated with a push event.
    pub fn new(
        repository: RepositoryInfo,
        pusher: UserInfo,
        sender: UserInfo,
        created: bool,
        deleted: bool,
        forced: bool,
    ) -> Self {
        Self {
            repository,
            pusher,
            sender,
            created,
            deleted,
            forced,
        }
    }
}

impl PushPayload {
    pub fn new(
        r#ref: String,
        before: String,
        after: String,
        metadata: PushMetadata,
        commits: Vec<CommitInfo>,
        head_commit: Option<CommitInfo>,
        compare: String,
    ) -> Self {
        Self {
            r#ref,
            before,
            after,
            repository: metadata.repository,
            pusher: metadata.pusher,
            sender: metadata.sender,
            created: metadata.created,
            deleted: metadata.deleted,
            forced: metadata.forced,
            commits,
            head_commit,
            compare,
        }
    }

    pub fn r#ref(&self) -> &str {
        &self.r#ref
    }

    pub fn before(&self) -> &str {
        &self.before
    }

    pub fn after(&self) -> &str {
        &self.after
    }

    pub fn repository(&self) -> &RepositoryInfo {
        &self.repository
    }

    pub fn pusher(&self) -> &UserInfo {
        &self.pusher
    }

    pub fn sender(&self) -> &UserInfo {
        &self.sender
    }

    pub fn created(&self) -> bool {
        self.created
    }

    pub fn deleted(&self) -> bool {
        self.deleted
    }

    pub fn forced(&self) -> bool {
        self.forced
    }

    pub fn commits(&self) -> &[CommitInfo] {
        &self.commits
    }

    pub fn head_commit(&self) -> &Option<CommitInfo> {
        &self.head_commit
    }

    pub fn compare(&self) -> &str {
        &self.compare
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> UserInfo {
        UserInfo::new("name".into(), "email".into(), "login".into())
    }

    fn repository() -> RepositoryInfo {
        RepositoryInfo::new(
            "repo".into(),
            "owner/repo".into(),
            user(),
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
        let commit = CommitInfo::new(
            "id".into(),
            "message".into(),
            "timestamp".into(),
            user(),
            user(),
            super::super::commit_info::CommitChanges::new(Vec::new(), Vec::new(), Vec::new()),
        );
        let payload = PushPayload::new(
            "refs/heads/main".into(),
            "before".into(),
            "after".into(),
            PushMetadata::new(repository(), user(), user(), true, false, true),
            vec![commit.clone()],
            Some(commit),
            "compare".into(),
        );

        assert_eq!(payload.r#ref(), "refs/heads/main");
        assert_eq!(payload.before(), "before");
        assert_eq!(payload.after(), "after");
        assert_eq!(payload.repository().name(), "repo");
        assert_eq!(payload.pusher().login(), "login");
        assert_eq!(payload.sender().login(), "login");
        assert!(payload.created());
        assert!(!payload.deleted());
        assert!(payload.forced());
        assert_eq!(payload.commits().len(), 1);
        assert_eq!(
            payload.head_commit().as_ref().map(CommitInfo::id),
            Some("id")
        );
        assert_eq!(payload.compare(), "compare");
    }
}
