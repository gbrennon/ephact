use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::expression::EvalContext;
use crate::domain::workflow::Step;

#[derive(Clone)]
pub struct RunActionRequest {
    pub action_ref: String,
    pub step: Step,
    pub repo_path: PathBuf,
    pub env: HashMap<String, String>,
    pub context: EvalContext,
    pub container: Arc<dyn ContainerPort>,
}

impl RunActionRequest {
    pub fn new(
        action_ref: String,
        step: Step,
        repo_path: PathBuf,
        env: HashMap<String, String>,
        context: EvalContext,
        container: Arc<dyn ContainerPort>,
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

    pub fn into_action_ref(self) -> String {
        self.action_ref
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn into_step(self) -> Step {
        self.step
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn into_repo_path(self) -> PathBuf {
        self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn into_env(self) -> HashMap<String, String> {
        self.env
    }

    pub fn context(&self) -> &EvalContext {
        &self.context
    }

    pub fn into_context(self) -> EvalContext {
        self.context
    }

    pub fn container(&self) -> &Arc<dyn ContainerPort> {
        &self.container
    }

    pub fn container_arc(&self) -> Arc<dyn ContainerPort> {
        self.container.clone()
    }

    pub fn into_container(self) -> Arc<dyn ContainerPort> {
        self.container
    }
}

impl fmt::Debug for RunActionRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunActionRequest")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}
