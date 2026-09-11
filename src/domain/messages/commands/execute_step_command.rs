use std::{collections::HashMap, fmt, path::PathBuf};

use crate::domain::{
    entities::Step, messages::commands::command::Command, value_objects::EvaluationContext,
};

/// Command representing the intention to execute one step of a job.
///
/// Published by the job coordination service for every step of the job, and
/// handled by the step command handler.
pub struct ExecuteStepCommand<'a, C: ?Sized + Sync + 'a> {
    step: Step,
    env: HashMap<String, String>,
    context: EvaluationContext,
    container: &'a C,
    repo_path: PathBuf,
}

impl<'a, C: ?Sized + Sync + 'a> ExecuteStepCommand<'a, C> {
    pub fn new(
        step: Step,
        env: HashMap<String, String>,
        context: EvaluationContext,
        container: &'a C,
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

    pub fn container(&self) -> &'a C {
        self.container
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
        &'a C,
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

impl<'a, C: ?Sized + Sync + 'a> Clone for ExecuteStepCommand<'a, C> {
    fn clone(&self) -> Self {
        Self {
            step: self.step.clone(),
            env: self.env.clone(),
            context: self.context.clone(),
            container: self.container,
            repo_path: self.repo_path.clone(),
        }
    }
}

impl<'a, C: ?Sized + Sync + 'a> fmt::Debug for ExecuteStepCommand<'a, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecuteStepCommand")
            .field("uses", &self.step.uses())
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}

impl<'a, C: ?Sized + Sync + 'a> Command for ExecuteStepCommand<'a, C> {}
