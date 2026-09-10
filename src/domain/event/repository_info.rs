use serde::Serialize;

use super::user_info::UserInfo;

/// Repository information included in event payloads.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RepositoryInfo {
    name: String,
    full_name: String,
    owner: UserInfo,
    private: bool,
    html_url: String,
    default_branch: String,
    clone_url: String,
    ssh_url: String,
}

/// Groups the URLs and default branch associated with a repository.
pub struct RepositoryLinks {
    html_url: String,
    default_branch: String,
    clone_url: String,
    ssh_url: String,
}

impl RepositoryLinks {
    /// Creates the links associated with a repository.
    pub fn new(
        html_url: String,
        default_branch: String,
        clone_url: String,
        ssh_url: String,
    ) -> Self {
        Self {
            html_url,
            default_branch,
            clone_url,
            ssh_url,
        }
    }
}

impl RepositoryInfo {
    pub fn new(
        name: String,
        full_name: String,
        owner: UserInfo,
        private: bool,
        links: RepositoryLinks,
    ) -> Self {
        Self {
            name,
            full_name,
            owner,
            private,
            html_url: links.html_url,
            default_branch: links.default_branch,
            clone_url: links.clone_url,
            ssh_url: links.ssh_url,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn owner(&self) -> &UserInfo {
        &self.owner
    }

    pub fn private(&self) -> bool {
        self.private
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }

    pub fn default_branch(&self) -> &str {
        &self.default_branch
    }

    pub fn clone_url(&self) -> &str {
        &self.clone_url
    }

    pub fn ssh_url(&self) -> &str {
        &self.ssh_url
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn owner() -> UserInfo {
        UserInfo::new("name".into(), "email".into(), "login".into())
    }

    #[test]
    fn new_preserves_fields() {
        let repository = RepositoryInfo::new(
            "repo".into(),
            "owner/repo".into(),
            owner(),
            true,
            RepositoryLinks::new("html".into(), "main".into(), "clone".into(), "ssh".into()),
        );

        assert_eq!(repository.name(), "repo");
        assert_eq!(repository.full_name(), "owner/repo");
        assert_eq!(repository.owner().login(), "login");
        assert!(repository.private());
        assert_eq!(repository.html_url(), "html");
        assert_eq!(repository.default_branch(), "main");
        assert_eq!(repository.clone_url(), "clone");
        assert_eq!(repository.ssh_url(), "ssh");
    }
}
