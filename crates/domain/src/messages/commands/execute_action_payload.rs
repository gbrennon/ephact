use std::{collections::HashMap, path::PathBuf};

use crate::{entities::Step, messages::Message, value_objects::EvaluationContext};

/// Data describing the intention to execute one action.
///
/// Published by the step coordination service and handled by the action command handler.
#[derive(Debug, Clone)]
pub struct ExecuteActionPayload {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
}

impl ExecuteActionPayload {
    pub fn new(
        action_ref: String,
        step: Step,
        repo_path: PathBuf,
        env: HashMap<String, String>,
        context: EvaluationContext,
    ) -> Self {
        Self {
            action_ref,
            step,
            repo_path,
            env,
            context,
        }
    }

    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        Step,
        PathBuf,
        HashMap<String, String>,
        EvaluationContext,
    ) {
        (
            self.action_ref,
            self.step,
            self.repo_path,
            self.env,
            self.context,
        )
    }
}

impl Message for ExecuteActionPayload {}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload_for_test() -> ExecuteActionPayload {
        ExecuteActionPayload::new(
            "owner/action@v1".into(),
            Step::new(None, None, None, Some("owner/action@v1".into())),
            PathBuf::from("/repo"),
            HashMap::from([("KEY".into(), "value".into())]),
            EvaluationContext::default(),
        )
    }

    #[test]
    fn new_exposes_fields() {
        let payload = payload_for_test();

        assert_eq!(payload.action_ref(), "owner/action@v1");
        assert_eq!(payload.step().uses(), Some("owner/action@v1"));
        assert_eq!(payload.repo_path(), &PathBuf::from("/repo"));
        assert_eq!(payload.env()["KEY"], "value");
        assert!(payload.context().get("environment").is_none());
    }

    #[test]
    fn new_stores_the_context() {
        let context = EvaluationContext::new()
            .with_root("environment", crate::value_objects::ContextValue::text("v"));

        let payload = ExecuteActionPayload::new(
            "owner/action@v1".into(),
            Step::new(None, None, None, Some("owner/action@v1".into())),
            PathBuf::from("/repo"),
            HashMap::new(),
            context,
        );

        assert_eq!(
            payload.context().get("environment"),
            Some(&crate::value_objects::ContextValue::text("v"))
        );
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (action_ref, step, repo_path, env, context) = payload_for_test().into_parts();

        assert_eq!(action_ref, "owner/action@v1");
        assert_eq!(step.uses(), Some("owner/action@v1"));
        assert_eq!(repo_path, PathBuf::from("/repo"));
        assert_eq!(env["KEY"], "value");
        assert!(context.get("environment").is_none());
    }

    #[test]
    fn debug_names_the_payload() {
        let rendered = format!("{:?}", payload_for_test());

        assert!(rendered.contains("ExecuteActionPayload"));
    }
}
