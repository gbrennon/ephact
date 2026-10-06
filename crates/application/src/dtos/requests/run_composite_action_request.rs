use std::{collections::HashMap, path::Path};

use crate::{domain::entities::Step, dtos::requests::ExecuteActionRequest};

/// Inputs for running the steps of a composite action.
pub struct RunCompositeActionRequest<'a> {
    steps: &'a [Step],
    inputs: &'a HashMap<String, String>,
    action_dir: &'a Path,
    action_request: &'a ExecuteActionRequest,
}

impl<'a> RunCompositeActionRequest<'a> {
    /// Creates inputs from steps, action inputs, an action directory, and an action request.
    pub fn new(
        steps: &'a [Step],
        inputs: &'a HashMap<String, String>,
        action_dir: &'a Path,
        action_request: &'a ExecuteActionRequest,
    ) -> Self {
        Self {
            steps,
            inputs,
            action_dir,
            action_request,
        }
    }

    /// Returns the steps.
    pub fn steps(&self) -> &'a [Step] {
        self.steps
    }

    /// Returns the action inputs.
    pub fn inputs(&self) -> &'a HashMap<String, String> {
        self.inputs
    }

    /// Returns the action directory.
    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }

    /// Returns the action request.
    pub fn action_request(&self) -> &'a ExecuteActionRequest {
        self.action_request
    }
}
