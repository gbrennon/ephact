use std::{error::Error, sync::Arc};

use crate::{
    application::{dtos::responses::WorkflowListItemResponse, ports::outbound::WorkflowSourcePort},
    domain::entities::repository::Repository,
};

#[derive(Clone)]
pub struct SharedWorkflowSource {
    inner: Arc<dyn WorkflowSourcePort>,
}

impl SharedWorkflowSource {
    pub fn new(inner: Arc<dyn WorkflowSourcePort>) -> Self {
        Self { inner }
    }
}

impl WorkflowSourcePort for SharedWorkflowSource {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        self.inner.read_workflow(repository, workflow_name)
    }

    fn read_all_workflows(&self, repository: &Repository) -> Result<Vec<String>, Box<dyn Error>> {
        self.inner.read_all_workflows(repository)
    }

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, Box<dyn Error>> {
        self.inner.list_actions(repository)
    }

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, Box<dyn Error>> {
        self.inner.list_workflows(repository)
    }
}
