/// Payload for [`DomainEvent::ActRunCompleted`].
///
/// [`DomainEvent::ActRunCompleted`]: super::domain_event::DomainEvent::ActRunCompleted
#[derive(Debug, Clone)]
pub struct ActRunCompletedPayload {
    run_id: String,
    repository_path: String,
    /// Names of containers created during the run. Handlers use these to
    /// stop (down), kill and remove containers without deleting cached images.
    container_names: Vec<String>,
    /// Whether the workflow succeeded.
    success: bool,
}

impl ActRunCompletedPayload {
    pub fn new(
        run_id: String,
        repository_path: String,
        container_names: Vec<String>,
        success: bool,
    ) -> Self {
        Self {
            run_id,
            repository_path,
            container_names,
            success,
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn repository_path(&self) -> &str {
        &self.repository_path
    }

    pub fn container_names(&self) -> &[String] {
        &self.container_names
    }

    pub fn success(&self) -> bool {
        self.success
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = ActRunCompletedPayload::new(
            "run-1".into(),
            "/repo".into(),
            vec!["container".into()],
            false,
        );

        assert_eq!(payload.run_id(), "run-1");
        assert_eq!(payload.repository_path(), "/repo");
        assert_eq!(payload.container_names(), ["container"]);
        assert!(!payload.success());
    }
}
