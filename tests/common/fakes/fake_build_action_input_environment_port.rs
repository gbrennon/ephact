use std::{collections::HashMap, sync::Arc};

use ephact::{
    application::dtos::{
        requests::BuildActionInputEnvironmentRequest,
        responses::BuildActionInputEnvironmentResponse,
    },
    infrastructure::actions::build_action_input_environment_port::BuildActionInputEnvironmentPort,
};
use parking_lot::Mutex;

/// Returns a prepared environment, recording the action paths it was given.
#[derive(Clone)]
pub struct FakeBuildActionInputEnvironmentPort {
    env: HashMap<String, String>,
    action_paths: Arc<Mutex<Vec<String>>>,
}

impl FakeBuildActionInputEnvironmentPort {
    pub fn returning(env: HashMap<String, String>) -> Self {
        Self {
            env,
            action_paths: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn action_paths(&self) -> Vec<String> {
        self.action_paths.lock().clone()
    }
}

impl BuildActionInputEnvironmentPort for FakeBuildActionInputEnvironmentPort {
    fn execute(
        &self,
        request: BuildActionInputEnvironmentRequest,
    ) -> BuildActionInputEnvironmentResponse {
        self.action_paths
            .lock()
            .push(request.action_path().to_string());
        BuildActionInputEnvironmentResponse::new(self.env.clone())
    }
}
