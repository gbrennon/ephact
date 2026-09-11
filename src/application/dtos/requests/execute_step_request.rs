use std::{collections::HashMap, path::Path, sync::Arc};

use crate::{
    application::ports::outbound::container_port::ContainerPort,
    domain::{entities::Step, value_objects::EvaluationContext},
};

pub struct ExecuteStepRequest<'a> {
    step: &'a Step,

    context: &'a EvaluationContext,

    container: Arc<dyn ContainerPort>,

    repo_path: &'a Path,

    env: &'a HashMap<String, String>,
}

impl<'a> ExecuteStepRequest<'a> {
    pub fn new(
        step: &'a Step,
        context: &'a EvaluationContext,
        container: Arc<dyn ContainerPort>,
        repo_path: &'a Path,
        env: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            step,
            context,
            container,
            repo_path,
            env,
        }
    }

    pub fn step(&self) -> &'a Step {
        self.step
    }

    pub fn context(&self) -> &'a EvaluationContext {
        self.context
    }

    pub fn container(&self) -> &Arc<dyn ContainerPort> {
        &self.container
    }

    pub fn container_arc(&self) -> Arc<dyn ContainerPort> {
        self.container.clone()
    }

    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }

    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }
}
