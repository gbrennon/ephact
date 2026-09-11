/// One workflow file discovered in a repository, as the presentation layer
/// displays it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkflowListItemResponse {
    name: Option<String>,
    file: Option<String>,
    events: Vec<String>,
}

impl WorkflowListItemResponse {
    pub fn new(name: Option<String>, file: Option<String>, events: Vec<String>) -> Self {
        Self { name, file, events }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }
    pub fn events(&self) -> &[String] {
        &self.events
    }

    pub fn has_pull_request_event(&self) -> bool {
        self.events.iter().any(|event| event == "pull_request")
    }

    pub fn into_parts(self) -> (Option<String>, Option<String>, Vec<String>) {
        (self.name, self.file, self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_workflow_list_item_keeps_fields() {
        let item = WorkflowListItemResponse::new(
            Some("ci".into()),
            Some("ci.yml".into()),
            vec!["pull_request".into()],
        );
        assert_eq!(item.name(), Some("ci"));
        assert_eq!(item.file(), Some("ci.yml"));
        assert_eq!(item.events(), &["pull_request".to_string()]);
    }

    #[test]
    fn new_workflow_list_item_allows_missing_fields() {
        let item = WorkflowListItemResponse::new(None, None, vec![]);
        assert_eq!(item.name(), None);
        assert_eq!(item.file(), None);
        assert!(item.events().is_empty());
    }

    #[test]
    fn has_pull_request_event_reports_pull_request_eligibility() {
        let item =
            WorkflowListItemResponse::new(None, None, vec!["push".into(), "pull_request".into()]);

        assert!(item.has_pull_request_event());
    }
}
