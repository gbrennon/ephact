use serde::Serialize;

use super::{label_info::LabelInfo, user_info::UserInfo};

/// Issue information.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct IssueInfo {
    number: u64,
    title: String,
    body: Option<String>,
    state: String,
    user: UserInfo,
    labels: Vec<LabelInfo>,
    html_url: String,
}

impl IssueInfo {
    pub fn new(
        number: u64,
        title: String,
        body: Option<String>,
        state: String,
        user: UserInfo,
        labels: Vec<LabelInfo>,
        html_url: String,
    ) -> Self {
        Self {
            number,
            title,
            body,
            state,
            user,
            labels,
            html_url,
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

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn user(&self) -> &UserInfo {
        &self.user
    }

    pub fn labels(&self) -> &[LabelInfo] {
        &self.labels
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let issue = IssueInfo::new(
            1,
            "title".into(),
            Some("body".into()),
            "open".into(),
            UserInfo::new("name".into(), "email".into(), "login".into()),
            vec![LabelInfo::new("bug".into(), "red".into())],
            "url".into(),
        );

        assert_eq!(issue.number(), 1);
        assert_eq!(issue.title(), "title");
        assert_eq!(issue.body(), Some("body"));
        assert_eq!(issue.state(), "open");
        assert_eq!(issue.user().login(), "login");
        assert_eq!(issue.labels()[0].name(), "bug");
        assert_eq!(issue.html_url(), "url");
    }
}
