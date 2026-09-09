/// Payload for [`DomainEvent::ActRunCompleted`].
///
/// [`DomainEvent::ActRunCompleted`]: super::domain_event::DomainEvent::ActRunCompleted
#[derive(Debug, Clone)]
pub struct ActRunCompletedPayload {
    /// Names of containers created during the run. Handlers use these to
    /// stop (down), kill and remove containers without deleting cached images.
    container_names: Vec<String>,
    /// Whether the workflow succeeded.
    success: bool,
}

impl ActRunCompletedPayload {
    pub fn new(container_names: Vec<String>, success: bool) -> Self {
        Self { container_names, success }
    }

    pub fn container_names(&self) -> &[String] {
        &self.container_names
    }

    pub fn success(&self) -> bool {
        self.success
    }
}
