use std::{collections::HashMap, fmt, path::PathBuf, sync::Arc};

use crate::{
    entities::Step, messages::commands::command::Command, value_objects::EvaluationContext,
};

/// Command representing the intention to execute one step of a job.
///
/// Published by the job coordination service for every step of the job, and
/// handled by the step command handler.
pub struct ExecuteStepCommand<C: ?Sized + Send + Sync> {
    step: Step,
    env: HashMap<String, String>,
    context: EvaluationContext,
    container: Arc<C>,
    repo_path: PathBuf,
}

impl<C: ?Sized + Send + Sync> ExecuteStepCommand<C> {
    pub fn new(
        step: Step,
        env: HashMap<String, String>,
        context: EvaluationContext,
        container: Arc<C>,
        repo_path: PathBuf,
    ) -> Self {
        Self {
            step,
            env,
            context,
            container,
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

    pub fn container(&self) -> &C {
        self.container.as_ref()
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn into_parts(
        self,
    ) -> (
        Step,
        HashMap<String, String>,
        EvaluationContext,
        Arc<C>,
        PathBuf,
    ) {
        (
            self.step,
            self.env,
            self.context,
            self.container,
            self.repo_path,
        )
    }
}

impl<C: ?Sized + Send + Sync> Clone for ExecuteStepCommand<C> {
    fn clone(&self) -> Self {
        Self {
            step: self.step.clone(),
            env: self.env.clone(),
            context: self.context.clone(),
            container: self.container.clone(),
            repo_path: self.repo_path.clone(),
        }
    }
}

impl<C: ?Sized + Send + Sync> fmt::Debug for ExecuteStepCommand<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecuteStepCommand")
            .field("uses", &self.step.uses())
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}

impl<C: ?Sized + Send + Sync> Command for ExecuteStepCommand<C> {}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_for_test() -> ExecuteStepCommand<()> {
        ExecuteStepCommand::new(
            Step::new(None, None, Some("echo hi".into()), None),
            HashMap::from([("KEY".into(), "value".into())]),
            EvaluationContext::new(),
            Arc::new(()),
            PathBuf::from("/repo"),
        )
    }

    #[test]
    fn new_exposes_every_field() {
        let command = command_for_test();

        assert_eq!(command.step().run(), Some("echo hi"));
        assert_eq!(command.env()["KEY"], "value");
        assert!(command.context().github().as_text().is_none());
        assert_eq!(command.container(), &());
        assert_eq!(command.repo_path(), &PathBuf::from("/repo"));
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (step, env, context, _container, repo_path) = command_for_test().into_parts();

        assert_eq!(step.run(), Some("echo hi"));
        assert_eq!(env["KEY"], "value");
        assert!(context.github().as_text().is_none());
        assert_eq!(repo_path, PathBuf::from("/repo"));
    }

    #[test]
    fn clone_preserves_fields() {
        let command = command_for_test().clone();

        assert_eq!(command.repo_path(), &PathBuf::from("/repo"));
    }

    #[test]
    fn debug_names_the_command() {
        let rendered = format!("{:?}", command_for_test());

        assert!(rendered.contains("ExecuteStepCommand"));
    }
}
