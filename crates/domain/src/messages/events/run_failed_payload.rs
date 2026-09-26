/// Payload emitted when a workflow run cannot complete.
#[derive(Debug, Clone)]
pub struct RunFailedPayload {
    run_id: String,
    repository_path: String,
    workflow_name: Option<String>,
    error: String,
}

impl RunFailedPayload {
    pub fn new(
        run_id: String,
        repository_path: String,
        workflow_name: Option<String>,
        error: String,
    ) -> Self {
        Self {
            run_id,
            repository_path,
            workflow_name,
            error,
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn repository_path(&self) -> &str {
        &self.repository_path
    }

    pub fn workflow_name(&self) -> Option<&str> {
        self.workflow_name.as_deref()
    }

    pub fn error(&self) -> &str {
        &self.error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = RunFailedPayload::new(
            "run-1".into(),
            "/repo".into(),
            Some("workflow".into()),
            "failure".into(),
        );

        assert_eq!(payload.run_id(), "run-1");
        assert_eq!(payload.repository_path(), "/repo");
        assert_eq!(payload.workflow_name(), Some("workflow"));
        assert_eq!(payload.error(), "failure");
    }
}
