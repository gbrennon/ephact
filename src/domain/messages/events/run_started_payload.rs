/// Payload emitted when a workflow run begins.
#[derive(Debug, Clone)]
pub struct RunStartedPayload {
    run_id: String,
    repository_path: String,
}

impl RunStartedPayload {
    pub fn new(run_id: String, repository_path: String) -> Self {
        Self {
            run_id,
            repository_path,
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn repository_path(&self) -> &str {
        &self.repository_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = RunStartedPayload::new("run-1".into(), "/repo".into());

        assert_eq!(payload.run_id(), "run-1");
        assert_eq!(payload.repository_path(), "/repo");
    }
}
