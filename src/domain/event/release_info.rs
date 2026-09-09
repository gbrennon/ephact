use serde::Serialize;

/// Release information.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReleaseInfo {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    draft: bool,
    prerelease: bool,
    html_url: String,
}

impl ReleaseInfo {
    pub fn new(tag_name: String, name: Option<String>, body: Option<String>, draft: bool, prerelease: bool, html_url: String) -> Self {
        Self { tag_name, name, body, draft, prerelease, html_url }
    }

    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn draft(&self) -> bool {
        self.draft
    }

    pub fn prerelease(&self) -> bool {
        self.prerelease
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }
}
