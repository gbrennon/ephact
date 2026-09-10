use crate::domain::{ActRunConfig, value_objects::ActEvent};

pub const PULL_REQUEST_EVENT_NAME: &str = "pull_request";

pub fn config_for_pull_request_event(config: ActRunConfig) -> ActRunConfig {
    config.with_event(ActEvent::new(PULL_REQUEST_EVENT_NAME.to_string()))
}
