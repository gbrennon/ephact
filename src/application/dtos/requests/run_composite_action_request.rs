use std::{collections::HashMap, path::Path};

use crate::application::dtos::requests::ExecuteActionRequest;
use crate::domain::entities::Step;

/// Everything needed to run the steps of a composite action.
pub struct RunCompositeActionRequest<'a> {
    steps: &'a [Step],
    inputs: &'a HashMap<String, String>,
    action_dir: &'a Path,
    action_request: &'a ExecuteActionRequest<'a>,
}

impl<'a> RunCompositeActionRequest<'a> {
    pub fn new(
        steps: &'a [Step],
        inputs: &'a HashMap<String, String>,
        action_dir: &'a Path,
        action_request: &'a ExecuteActionRequest<'a>,
    ) -> Self {
        Self {
            steps,
            inputs,
            action_dir,
            action_request,
        }
    }

    pub fn steps(&self) -> &'a [Step] {
        self.steps
    }

    pub fn inputs(&self) -> &'a HashMap<String, String> {
        self.inputs
    }

    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }

    pub fn action_request(&self) -> &'a ExecuteActionRequest<'a> {
        self.action_request
    }
}
