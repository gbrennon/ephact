use std::{collections::HashMap, fmt, path::PathBuf, sync::Arc};

use crate::{entities::Step, messages::Message, value_objects::EvaluationContext};

/// Command representing the intention to execute one action.
///
/// Published by the step coordination service and handled by the action command handler.
pub struct ExecuteActionCommand<C: ?Sized + Send + Sync> {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
    container: Arc<C>,
}

impl<C: ?Sized + Send + Sync> ExecuteActionCommand<C> {
    pub fn new(
        action_ref: String,
        step: Step,
        repo_path: PathBuf,
        env: HashMap<String, String>,
        container: Arc<C>,
    ) -> Self {
        Self {
            action_ref,
            step,
            repo_path,
            env,
            context: EvaluationContext::default(),
            container,
        }
    }

    pub fn with_context(mut self, context: EvaluationContext) -> Self {
        self.context = context;
        self
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

    pub fn container(&self) -> &C {
        self.container.as_ref()
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        Step,
        PathBuf,
        HashMap<String, String>,
        EvaluationContext,
        Arc<C>,
    ) {
        (
            self.action_ref,
            self.step,
            self.repo_path,
            self.env,
            self.context,
            self.container,
        )
    }
}

impl<C: ?Sized + Send + Sync> Clone for ExecuteActionCommand<C> {
    fn clone(&self) -> Self {
        Self {
            action_ref: self.action_ref.clone(),
            step: self.step.clone(),
            repo_path: self.repo_path.clone(),
            env: self.env.clone(),
            context: self.context.clone(),
            container: self.container.clone(),
        }
    }
}

impl<C: ?Sized + Send + Sync> fmt::Debug for ExecuteActionCommand<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecuteActionCommand")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}

impl<C: ?Sized + Send + Sync> Message for ExecuteActionCommand<C> {}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_for_test() -> ExecuteActionCommand<()> {
        ExecuteActionCommand::new(
            "owner/action@v1".into(),
            Step::new(None, None, None, Some("owner/action@v1".into())),
            PathBuf::from("/repo"),
            HashMap::from([("KEY".into(), "value".into())]),
            Arc::new(()),
        )
    }

    #[test]
    fn new_defaults_context_and_exposes_fields() {
        let command = command_for_test();

        assert_eq!(command.action_ref(), "owner/action@v1");
        assert_eq!(command.step().uses(), Some("owner/action@v1"));
        assert_eq!(command.repo_path(), &PathBuf::from("/repo"));
        assert_eq!(command.env()["KEY"], "value");
        assert!(command.context().get("environment").is_none());
        assert_eq!(command.container(), &());
    }

    #[test]
    fn with_context_replaces_the_context() {
        let context = EvaluationContext::new()
            .with_root("environment", crate::value_objects::ContextValue::text("v"));

        let command = command_for_test().with_context(context.clone());

        assert_eq!(
            command.context().get("environment"),
            Some(&crate::value_objects::ContextValue::text("v"))
        );
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (action_ref, step, repo_path, env, context, _container) =
            command_for_test().into_parts();

        assert_eq!(action_ref, "owner/action@v1");
        assert_eq!(step.uses(), Some("owner/action@v1"));
        assert_eq!(repo_path, PathBuf::from("/repo"));
        assert_eq!(env["KEY"], "value");
        assert!(context.get("environment").is_none());
    }

    #[test]
    fn clone_preserves_fields() {
        let command = command_for_test().clone();

        assert_eq!(command.action_ref(), "owner/action@v1");
    }

    #[test]
    fn debug_names_the_command() {
        let rendered = format!("{:?}", command_for_test());

        assert!(rendered.contains("ExecuteActionCommand"));
    }
}
