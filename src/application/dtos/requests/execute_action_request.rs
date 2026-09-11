use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use crate::{
    application::ports::outbound::container_port::ContainerPort,
    domain::{entities::Step, value_objects::EvaluationContext},
};

#[derive(Clone)]
pub struct ExecuteActionRequest<'a> {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvaluationContext,
    container: &'a dyn ContainerPort,
}

impl<'a> ExecuteActionRequest<'a> {
    pub fn new(
        action_ref: impl Into<String>,
        step: Step,
        repo_path: impl Into<PathBuf>,
        env: HashMap<String, String>,
        context: EvaluationContext,
        container: &'a dyn ContainerPort,
    ) -> Self {
        Self {
            action_ref: action_ref.into(),
            step,
            repo_path: repo_path.into(),
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

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn container(&self) -> &'a dyn ContainerPort {
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
        &'a dyn ContainerPort,
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

impl fmt::Debug for ExecuteActionRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecuteActionRequest")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}
