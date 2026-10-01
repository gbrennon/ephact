use std::collections::BTreeMap;

use crate::{
    application::{
        dtos::{requests::BuildRunContextRequest, responses::BuildRunContextResponse},
        ports::outbound::BuildRunContextPort,
    },
    containers::workspace::CONTAINER_WORKSPACE,
    domain::value_objects::{ContextValue, EvaluationContext},
};

pub struct BuildRunContextService;

impl BuildRunContextService {
    pub fn new() -> Self {
        Self
    }

    /// Returns the runner facts every job sees in the `runner` context.
    fn runner_context(&self) -> ContextValue {
        ContextValue::mapping([
            ("os".to_owned(), ContextValue::text("Linux")),
            ("arch".to_owned(), ContextValue::text("X64")),
            ("temp".to_owned(), ContextValue::text("/tmp")),
        ])
    }
}

impl Default for BuildRunContextService {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildRunContextPort for BuildRunContextService {
    fn build(&self, request: BuildRunContextRequest) -> BuildRunContextResponse {
        let secrets = ContextValue::mapping(request.config().secrets().iter().map(|secret| {
            (
                secret.name().to_string(),
                ContextValue::text(secret.value()),
            )
        }));
        let inputs: BTreeMap<String, ContextValue> = request
            .config()
            .inputs()
            .iter()
            .map(|input| (input.key().to_string(), ContextValue::text(input.value())))
            .collect();
        let event_name = request
            .config()
            .event()
            .map_or("workflow_dispatch", |event| event.as_str());

        let event =
            ContextValue::mapping([("inputs".to_owned(), ContextValue::Mapping(inputs.clone()))]);

        let github = ContextValue::mapping([
            ("event_name".to_owned(), ContextValue::text(event_name)),
            (
                "repository".to_owned(),
                ContextValue::text(request.repository().name().as_str()),
            ),
            (
                "workspace".to_owned(),
                ContextValue::text(CONTAINER_WORKSPACE),
            ),
            ("event".to_owned(), event),
        ]);

        let context = EvaluationContext::new()
            .with_root("secrets", secrets)
            .with_root("inputs", ContextValue::Mapping(inputs))
            .with_root("github", github)
            .with_root("runner", self.runner_context());
        BuildRunContextResponse::new(context)
    }
}
