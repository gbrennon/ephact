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
    pub fn new(number: u64, title: String, body: Option<String>, state: String, user: UserInfo, labels: Vec<LabelInfo>, html_url: String) -> Self {
        Self { number, title, body, state, user, labels, html_url }
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
