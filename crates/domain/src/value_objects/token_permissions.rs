/// TokenPermissions for the workflow token in a workflow or job.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TokenPermissions {
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
}

impl TokenPermissions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_actions(mut self, actions: Option<String>) -> Self {
        self.actions = actions;
        self
    }

    pub fn with_checks(mut self, checks: Option<String>) -> Self {
        self.checks = checks;
        self
    }

    pub fn with_contents(mut self, contents: Option<String>) -> Self {
        self.contents = contents;
        self
    }

    pub fn with_deployments(mut self, deployments: Option<String>) -> Self {
        self.deployments = deployments;
        self
    }

    pub fn with_issues(mut self, issues: Option<String>) -> Self {
        self.issues = issues;
        self
    }

    pub fn with_packages(mut self, packages: Option<String>) -> Self {
        self.packages = packages;
        self
    }

    pub fn with_pages(mut self, pages: Option<String>) -> Self {
        self.pages = pages;
        self
    }

    pub fn with_pull_requests(mut self, pull_requests: Option<String>) -> Self {
        self.pull_requests = pull_requests;
        self
    }

    pub fn with_repository_projects(mut self, repository_projects: Option<String>) -> Self {
        self.repository_projects = repository_projects;
        self
    }

    pub fn with_security_events(mut self, security_events: Option<String>) -> Self {
        self.security_events = security_events;
        self
    }

    pub fn with_statuses(mut self, statuses: Option<String>) -> Self {
        self.statuses = statuses;
        self
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
        let permissions = TokenPermissions::new()
            .with_actions(Some("read".into()))
            .with_checks(Some("read".into()))
            .with_contents(Some("write".into()))
            .with_deployments(Some("none".into()))
            .with_issues(Some("read".into()))
            .with_packages(Some("write".into()))
            .with_pages(Some("read".into()))
            .with_pull_requests(Some("write".into()))
            .with_repository_projects(Some("read".into()))
            .with_security_events(Some("none".into()))
            .with_statuses(Some("read".into()));

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
