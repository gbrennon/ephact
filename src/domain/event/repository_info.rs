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

impl RepositoryInfo {
    pub fn new(name: String, full_name: String, owner: UserInfo, private: bool, html_url: String, default_branch: String, clone_url: String, ssh_url: String) -> Self {
        Self { name, full_name, owner, private, html_url, default_branch, clone_url, ssh_url }
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
