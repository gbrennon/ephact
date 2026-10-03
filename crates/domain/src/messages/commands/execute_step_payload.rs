use std::{collections::HashMap, path::PathBuf};

use crate::{entities::Step, messages::Message, value_objects::EvaluationContext};

/// Data describing the intention to execute one step.
#[derive(Debug, Clone)]
pub struct ExecuteStepPayload {
    step: Step,
    env: HashMap<String, String>,
    context: EvaluationContext,
    repo_path: PathBuf,
}

impl ExecuteStepPayload {
    pub fn new(
        step: Step,
        env: HashMap<String, String>,
        context: EvaluationContext,
        repo_path: PathBuf,
    ) -> Self {
        Self {
            step,
            env,
            context,
            repo_path,
        }
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn into_parts(self) -> (Step, HashMap<String, String>, EvaluationContext, PathBuf) {
        (self.step, self.env, self.context, self.repo_path)
    }
}

impl Message for ExecuteStepPayload {}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload_for_test() -> ExecuteStepPayload {
        ExecuteStepPayload::new(
            Step::new(None, None, Some("echo hi".into()), None),
            HashMap::from([("KEY".into(), "value".into())]),
            EvaluationContext::new(),
            PathBuf::from("/repo"),
        )
    }

    #[test]
    fn new_exposes_every_field() {
        let payload = payload_for_test();

        assert_eq!(payload.step().run(), Some("echo hi"));
        assert_eq!(payload.env()["KEY"], "value");
        assert!(payload.context().get("source").is_none());
        assert_eq!(payload.repo_path(), &PathBuf::from("/repo"));
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (step, env, context, repo_path) = payload_for_test().into_parts();

        assert_eq!(step.run(), Some("echo hi"));
        assert_eq!(env["KEY"], "value");
        assert!(context.get("source").is_none());
        assert_eq!(repo_path, PathBuf::from("/repo"));
    }

    #[test]
    fn debug_names_the_payload() {
        let rendered = format!("{:?}", payload_for_test());

        assert!(rendered.contains("ExecuteStepPayload"));
    }
}
