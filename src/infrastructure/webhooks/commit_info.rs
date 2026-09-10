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

/// Groups the file changes associated with a commit.
pub struct CommitChanges {
    added: Vec<String>,
    removed: Vec<String>,
    modified: Vec<String>,
}

impl CommitChanges {
    /// Creates a commit's added, removed, and modified file lists.
    pub fn new(added: Vec<String>, removed: Vec<String>, modified: Vec<String>) -> Self {
        Self {
            added,
            removed,
            modified,
        }
    }
}

impl CommitInfo {
    pub fn new(
        id: String,
        message: String,
        timestamp: String,
        author: UserInfo,
        committer: UserInfo,
        changes: CommitChanges,
    ) -> Self {
        Self {
            id,
            message,
            timestamp,
            author,
            committer,
            added: changes.added,
            removed: changes.removed,
            modified: changes.modified,
        }
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
#[cfg(test)]
mod tests {
    use super::*;
    fn user() -> UserInfo {
        UserInfo::new("name".into(), "email".into(), "login".into())
    }

    #[test]
    fn new_preserves_fields() {
        let commit = CommitInfo::new(
            "id".into(),
            "message".into(),
            "timestamp".into(),
            user(),
            user(),
            CommitChanges::new(
                vec!["added".into()],
                vec!["removed".into()],
                vec!["modified".into()],
            ),
        );

        assert_eq!(commit.id(), "id");
        assert_eq!(commit.message(), "message");
        assert_eq!(commit.timestamp(), "timestamp");
        assert_eq!(commit.author().login(), "login");
        assert_eq!(commit.committer().login(), "login");
        assert_eq!(commit.added(), &["added".to_string()]);
        assert_eq!(commit.removed(), &["removed".to_string()]);
        assert_eq!(commit.modified(), &["modified".to_string()]);
    }
}
