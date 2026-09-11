use std::{collections::HashMap, fmt, path::PathBuf};

use crate::domain::{
    entities::Step, messages::commands::command::Command, value_objects::EvaluationContext,
};

/// Command representing the intention to execute one action.
///
/// Published by the step coordination service and handled by the action command handler.
pub struct ExecuteActionCommand<'a, C: ?Sized + Sync + 'a> {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
    container: &'a C,
}

impl<'a, C: ?Sized + Sync + 'a> ExecuteActionCommand<'a, C> {
    pub fn new(
        action_ref: String,
        step: Step,
        repo_path: PathBuf,
        env: HashMap<String, String>,
        context: EvaluationContext,
        container: &'a C,
    ) -> Self {
        Self {
            action_ref,
            step,
            repo_path,
            env,
            context,
            container,
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

    pub fn container(&self) -> &'a C {
        self.container
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        Step,
        PathBuf,
        HashMap<String, String>,
        EvaluationContext,
        &'a C,
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

impl<'a, C: ?Sized + Sync + 'a> Clone for ExecuteActionCommand<'a, C> {
    fn clone(&self) -> Self {
        Self {
            action_ref: self.action_ref.clone(),
            step: self.step.clone(),
            repo_path: self.repo_path.clone(),
            env: self.env.clone(),
            context: self.context.clone(),
            container: self.container,
        }
    }
}

impl<'a, C: ?Sized + Sync + 'a> fmt::Debug for ExecuteActionCommand<'a, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecuteActionCommand")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}

impl<'a, C: ?Sized + Sync + 'a> Command for ExecuteActionCommand<'a, C> {}
