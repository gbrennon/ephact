use serde::Deserialize;

/// Permissions for the `GITHUB_TOKEN` in a workflow or job.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Permissions {
    actions: Option<String>,
    checks: Option<String>,
    contents: Option<String>,
    deployments: Option<String>,
    issues: Option<String>,
    packages: Option<String>,
    pages: Option<String>,
    #[serde(rename = "pull-requests")]
    pull_requests: Option<String>,
    #[serde(rename = "repository-projects")]
    repository_projects: Option<String>,
    #[serde(rename = "security-events")]
    security_events: Option<String>,
    statuses: Option<String>,
}

impl Permissions {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        actions: Option<String>,
        checks: Option<String>,
        contents: Option<String>,
        deployments: Option<String>,
        issues: Option<String>,
        packages: Option<String>,
        pages: Option<String>,
        pull_requests: Option<String>,
        repository_projects: Option<String>,
        security_events: Option<String>,
        statuses: Option<String>,
    ) -> Self {
        Self {
            actions,
            checks,
            contents,
            deployments,
            issues,
            packages,
            pages,
            pull_requests,
            repository_projects,
            security_events,
            statuses,
        }
    }

    pub fn actions(&self) -> Option<&str> {
        self.actions.as_deref()
    }

    pub fn checks(&self) -> Option<&str> {
        self.checks.as_deref()
    }

    pub fn contents(&self) -> Option<&str> {
        self.contents.as_deref()
    }

    pub fn deployments(&self) -> Option<&str> {
        self.deployments.as_deref()
    }

    pub fn issues(&self) -> Option<&str> {
        self.issues.as_deref()
    }

    pub fn packages(&self) -> Option<&str> {
        self.packages.as_deref()
    }

    pub fn pages(&self) -> Option<&str> {
        self.pages.as_deref()
    }

    pub fn pull_requests(&self) -> Option<&str> {
        self.pull_requests.as_deref()
    }

    pub fn repository_projects(&self) -> Option<&str> {
        self.repository_projects.as_deref()
    }

    pub fn security_events(&self) -> Option<&str> {
        self.security_events.as_deref()
    }

    pub fn statuses(&self) -> Option<&str> {
        self.statuses.as_deref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_all_permissions() {
        let permissions = Permissions::new(
            Some("read".into()),
            Some("read".into()),
            Some("write".into()),
            Some("none".into()),
            Some("read".into()),
            Some("write".into()),
            Some("read".into()),
            Some("write".into()),
            Some("read".into()),
            Some("none".into()),
            Some("read".into()),
        );

        assert_eq!(permissions.actions(), Some("read"));
        assert_eq!(permissions.checks(), Some("read"));
        assert_eq!(permissions.contents(), Some("write"));
        assert_eq!(permissions.deployments(), Some("none"));
        assert_eq!(permissions.issues(), Some("read"));
        assert_eq!(permissions.packages(), Some("write"));
        assert_eq!(permissions.pages(), Some("read"));
        assert_eq!(permissions.pull_requests(), Some("write"));
        assert_eq!(permissions.repository_projects(), Some("read"));
        assert_eq!(permissions.security_events(), Some("none"));
        assert_eq!(permissions.statuses(), Some("read"));
    }
}
