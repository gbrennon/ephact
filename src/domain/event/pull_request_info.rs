use serde::Serialize;

use super::{branch_ref::BranchRef, user_info::UserInfo};

/// Pull request information.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PullRequestInfo {
    number: u64,
    title: String,
    body: Option<String>,
    head: BranchRef,
    base: BranchRef,
    user: UserInfo,
    html_url: String,
    draft: bool,
    merged: bool,
    mergeable: Option<bool>,
}

impl PullRequestInfo {
    pub fn new(number: u64, title: String, body: Option<String>, head: BranchRef, base: BranchRef, user: UserInfo, html_url: String, draft: bool, merged: bool, mergeable: Option<bool>) -> Self {
        Self { number, title, body, head, base, user, html_url, draft, merged, mergeable }
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn head(&self) -> &BranchRef {
        &self.head
    }

    pub fn base(&self) -> &BranchRef {
        &self.base
    }

    pub fn user(&self) -> &UserInfo {
        &self.user
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }

    pub fn draft(&self) -> bool {
        self.draft
    }

    pub fn merged(&self) -> bool {
        self.merged
    }

    pub fn mergeable(&self) -> Option<bool> {
        self.mergeable
    }
}
