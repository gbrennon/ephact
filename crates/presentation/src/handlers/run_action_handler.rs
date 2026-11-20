use std::{collections::HashMap, path::PathBuf};

use crate::application::{
    dtos::{
        requests::{RunActionRequest, RunActionRequestInput},
        responses::ExecuteActionResponse,
    },
    ports::{inbound::RunActionPort, outbound::StepTextCodecPort},
};

/// Handles execution of one selected action without constructing a workflow.
pub struct RunActionHandler;

impl RunActionHandler {
    /// Executes one action through the application action port.
    pub async fn handle(
        port: &dyn RunActionPort,
        step_codec: &dyn StepTextCodecPort,
        action_ref: String,
        inputs: Vec<(String, String)>,
        repository_path: PathBuf,
    ) -> Result<ExecuteActionResponse, Box<dyn std::error::Error>> {
        let inputs = inputs.into_iter().collect::<HashMap<_, _>>();
        let input =
            RunActionRequestInput::for_action(action_ref, inputs, repository_path, step_codec)?;
        Ok(port.execute(RunActionRequest::new(input))?)
    }
}
