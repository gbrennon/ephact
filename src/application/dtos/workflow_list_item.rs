/// One workflow file discovered in a repository, as the presentation layer
/// displays it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkflowListItem {
    name: Option<String>,
    file: Option<String>,
}

impl WorkflowListItem {
    pub fn new(name: Option<String>, file: Option<String>) -> Self {
        Self { name, file }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    pub fn into_parts(self) -> (Option<String>, Option<String>) {
        (self.name, self.file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_workflow_list_item_keeps_fields() {
        let item = WorkflowListItem::new(Some("ci".into()), Some("ci.yml".into()));
        assert_eq!(item.name(), Some("ci"));
        assert_eq!(item.file(), Some("ci.yml"));
    }

    #[test]
    fn new_workflow_list_item_allows_missing_fields() {
        let item = WorkflowListItem::new(None, None);
        assert_eq!(item.name(), None);
        assert_eq!(item.file(), None);
    }
}
