use serde::Serialize;

use super::{commit_info::CommitInfo, repository_info::RepositoryInfo, user_info::UserInfo};

/// Payload for `push` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PushPayload {
    pub r#ref: String,
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

impl PushPayload {
    pub fn new(r#ref: String, before: String, after: String, repository: RepositoryInfo, pusher: UserInfo, sender: UserInfo, created: bool, deleted: bool, forced: bool, commits: Vec<CommitInfo>, head_commit: Option<CommitInfo>, compare: String) -> Self {
        Self { r#ref, before, after, repository, pusher, sender, created, deleted, forced, commits, head_commit, compare }
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
