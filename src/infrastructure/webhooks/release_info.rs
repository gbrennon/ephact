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
    pub fn new(
        tag_name: String,
        name: Option<String>,
        body: Option<String>,
        draft: bool,
        prerelease: bool,
        html_url: String,
    ) -> Self {
        Self {
            tag_name,
            name,
            body,
            draft,
            prerelease,
            html_url,
        }
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let release = ReleaseInfo::new(
            "v1".into(),
            Some("Release".into()),
            Some("Notes".into()),
            true,
            false,
            "url".into(),
        );

        assert_eq!(release.tag_name(), "v1");
        assert_eq!(release.name(), Some("Release"));
        assert_eq!(release.body(), Some("Notes"));
        assert!(release.draft());
        assert!(!release.prerelease());
        assert_eq!(release.html_url(), "url");
    }
}
