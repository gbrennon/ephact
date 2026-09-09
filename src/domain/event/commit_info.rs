use serde::Serialize;

use super::user_info::UserInfo;

/// Commit information for push events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CommitInfo {
    id: String,
    message: String,
    timestamp: String,
    author: UserInfo,
    committer: UserInfo,
    added: Vec<String>,
    removed: Vec<String>,
    modified: Vec<String>,
}

impl CommitInfo {
    pub fn new(id: String, message: String, timestamp: String, author: UserInfo, committer: UserInfo, added: Vec<String>, removed: Vec<String>, modified: Vec<String>) -> Self {
        Self { id, message, timestamp, author, committer, added, removed, modified }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn author(&self) -> &UserInfo {
        &self.author
    }

    pub fn committer(&self) -> &UserInfo {
        &self.committer
    }

    pub fn added(&self) -> &[String] {
        &self.added
    }

    pub fn removed(&self) -> &[String] {
        &self.removed
    }

    pub fn modified(&self) -> &[String] {
        &self.modified
    }
}
