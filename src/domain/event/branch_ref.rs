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
        Self { r#ref, sha, repo, label }
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
