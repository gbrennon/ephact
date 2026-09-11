#![allow(dead_code)]
use std::{collections::HashMap, path::PathBuf};

use ephact::application::dtos::requests::ExecuteActionRequest;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::value_objects::EvaluationContext;
use ephact::infrastructure::workflows::yaml::StepYaml;

/// Builds action execution requests for tests that only care about which
/// action was requested.
pub struct ActionRequestFixture;

impl ActionRequestFixture {
    pub fn for_action<'a>(
        action_ref: &str,
        container: &'a dyn ContainerPort,
    ) -> ExecuteActionRequest<'a> {
        ExecuteActionRequest::new(
            action_ref,
            serde_yaml::from_str::<StepYaml>(&format!("uses: {action_ref}\n"))
                .unwrap()
                .into_domain(),
            PathBuf::from("/workspace"),
            HashMap::new(),
            EvaluationContext::new(),
            container,
        )
    }
}
