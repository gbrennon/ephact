/// Payload emitted when a container has been created for a workflow run.
///
/// Carries the run it belongs to and the name of the created container.
#[derive(Debug, Clone)]
pub struct ContainerStartedPayload {
    run_id: String,
    container_name: String,
}

impl ContainerStartedPayload {
    pub fn new(run_id: String, container_name: String) -> Self {
        Self {
            run_id,
            container_name,
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = ContainerStartedPayload::new("run-1".into(), "ephemeral-act-build".into());

        assert_eq!(payload.run_id(), "run-1");
        assert_eq!(payload.container_name(), "ephemeral-act-build");
    }
}
