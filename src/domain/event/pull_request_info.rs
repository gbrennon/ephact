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
    pub fn new(
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
    ) -> Self {
        Self {
            number,
            title,
            body,
            head,
            base,
            user,
            html_url,
            draft,
            merged,
            mergeable,
        }
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::event::RepositoryInfo;
    use crate::domain::event::UserInfo;

    fn branch() -> BranchRef {
        BranchRef::new(
            "refs/heads/main".into(),
            "sha".into(),
            RepositoryInfo::new(
                "repo".into(),
                "owner/repo".into(),
                UserInfo::new("name".into(), "email".into(), "login".into()),
                false,
                "html".into(),
                "main".into(),
                "clone".into(),
                "ssh".into(),
            ),
            "main".into(),
        )
    }

    #[test]
    fn new_preserves_fields() {
        let info = PullRequestInfo::new(
            1,
            "title".into(),
            Some("body".into()),
            branch(),
            branch(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            "url".into(),
            true,
            true,
            Some(true),
        );

        assert_eq!(info.number(), 1);
        assert_eq!(info.title(), "title");
        assert_eq!(info.body(), Some("body"));
        assert_eq!(info.head().sha(), "sha");
        assert_eq!(info.base().label(), "main");
        assert_eq!(info.user().login(), "login");
        assert_eq!(info.html_url(), "url");
        assert!(info.draft());
        assert!(info.merged());
        assert_eq!(info.mergeable(), Some(true));
    }
}
