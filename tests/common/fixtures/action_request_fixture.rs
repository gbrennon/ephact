#![allow(dead_code)]
use std::{collections::HashMap, path::PathBuf};

use ephact::{
    application::{
        dtos::requests::{
            ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
        },
        ports::outbound::container_port::ContainerPort,
    },
    domain::{
        services::{evaluation_context_mapper::EvaluationContextMapper, step_factory::StepFactory},
        value_objects::EvaluationContext,
    },
    infrastructure::workflows::yaml::StepYaml,
};

/// Builds action execution requests for tests that only care about which
/// action was requested.
pub struct ActionRequestFixture;

impl ActionRequestFixture {
    pub fn for_action(action_ref: &str, _container: &dyn ContainerPort) -> ExecuteActionRequest {
        ExecuteActionRequest::new(ExecuteActionRequestInput::new(
            action_ref,
            StepFactory::to_text(
                &serde_yaml::from_str::<StepYaml>(&format!("uses: {action_ref}\n"))
                    .unwrap()
                    .into_domain(),
            )
            .unwrap(),
            ExecuteActionExecutionInput::new(
                PathBuf::from("/workspace"),
                HashMap::new(),
                EvaluationContextMapper::to_parts(&EvaluationContext::new()),
            ),
        ))
    }
}
