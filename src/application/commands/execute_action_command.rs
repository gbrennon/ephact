use std::{collections::HashMap, fmt, path::PathBuf, sync::Arc};

use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::expression::EvalContext;
use crate::domain::workflow::Step;

#[derive(Clone)]
pub struct ExecuteActionCommand {
    action_ref: String,
    step: Step,
    repo_path: PathBuf,
    env: HashMap<String, String>,
    context: EvalContext,
    container: Arc<dyn ContainerPort>,
}

impl ExecuteActionCommand {
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

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn context(&self) -> &EvalContext {
        &self.context
    }

    pub fn container(&self) -> &Arc<dyn ContainerPort> {
        &self.container
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        Step,
        PathBuf,
        HashMap<String, String>,
        EvalContext,
        Arc<dyn ContainerPort>,
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
impl fmt::Debug for ExecuteActionCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecuteActionCommand")
            .field("action_ref", &self.action_ref)
            .field("repo_path", &self.repo_path)
            .finish_non_exhaustive()
    }
}
