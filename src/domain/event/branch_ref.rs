use serde::Serialize;

use super::repository_info::RepositoryInfo;

/// Branch reference for pull requests.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BranchRef {
    r#ref: String,
    sha: String,
    repo: RepositoryInfo,
    label: String,
}

impl BranchRef {
    pub fn new(r#ref: String, sha: String, repo: RepositoryInfo, label: String) -> Self {
        Self {
            r#ref,
            sha,
            repo,
            label,
        }
    }

    pub fn r#ref(&self) -> &str {
        &self.r#ref
    }

    pub fn sha(&self) -> &str {
        &self.sha
    }

    pub fn repo(&self) -> &RepositoryInfo {
        &self.repo
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::event::UserInfo;

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
        let branch = BranchRef::new(
            "refs/heads/main".into(),
            "sha".into(),
            repository(),
            "main".into(),
        );

        assert_eq!(branch.r#ref(), "refs/heads/main");
        assert_eq!(branch.sha(), "sha");
        assert_eq!(branch.repo().name(), "repo");
        assert_eq!(branch.label(), "main");
    }
}
