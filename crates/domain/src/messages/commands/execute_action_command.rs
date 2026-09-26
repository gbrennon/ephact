use std::{collections::HashMap, fmt, path::PathBuf, sync::Arc};

use crate::{
    entities::Step, messages::commands::command::Command, value_objects::EvaluationContext,
};

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

impl<C: ?Sized + Send + Sync> Command for ExecuteActionCommand<C> {}
