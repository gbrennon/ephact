#![allow(dead_code)]
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use ephact::{
    application::{dtos::ExecuteActionRequest, ports::outbound::container_port::ContainerPort},
    domain::value_objects::EvaluationContext,
};

use crate::common::fakes::stub_container::StubContainer;
use ephact::infrastructure::workflows::yaml::StepYaml;

/// Builds action execution requests for tests that only care about which
/// action was requested.
pub struct ActionRequestFixture;

impl ActionRequestFixture {
    pub fn for_action(action_ref: &str) -> ExecuteActionRequest {
        let container: Arc<dyn ContainerPort> = Arc::new(StubContainer);
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
