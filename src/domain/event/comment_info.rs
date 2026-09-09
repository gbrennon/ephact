use serde::Serialize;

use super::user_info::UserInfo;

/// Comment information.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CommentInfo {
    id: u64,
    body: String,
    user: UserInfo,
    html_url: String,
}

impl CommentInfo {
    pub fn new(id: u64, body: String, user: UserInfo, html_url: String) -> Self {
        Self { id, body, user, html_url }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn user(&self) -> &UserInfo {
        &self.user
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }
}
