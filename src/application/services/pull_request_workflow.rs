use crate::domain::{ActRunConfig, value_objects::ActEvent, workflow::Workflow};

pub const PULL_REQUEST_EVENT_NAME: &str = "pull_request";

pub fn content_has_pull_request_event(content: &str) -> bool {
    serde_yaml::from_str::<Workflow>(content)
        .map(|workflow| workflow.on().has_event(PULL_REQUEST_EVENT_NAME))
        .unwrap_or(false)
}

pub fn config_for_pull_request_event(config: ActRunConfig) -> ActRunConfig {
    config.with_event(ActEvent::new(PULL_REQUEST_EVENT_NAME.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_has_pull_request_event_accepts_pull_request_workflow() {
        let content = "on: pull_request\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - run: cargo test\n";

        assert!(content_has_pull_request_event(content));
    }

    #[test]
    fn content_has_pull_request_event_rejects_push_workflow() {
        let content = "on: push\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - run: cargo test\n";

        assert!(!content_has_pull_request_event(content));
    }
}
