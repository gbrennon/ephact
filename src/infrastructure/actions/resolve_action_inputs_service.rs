use std::collections::HashMap;

use crate::application::dtos::requests::ResolveActionInputsRequest;
use crate::application::ports::outbound::resolve_action_inputs_port::ResolveActionInputsPort;
use crate::domain::errors::StepError;

/// Service that resolves the inputs an action runs with, overlaying the step's
/// `with:` values on the defaults the action declared.
pub struct ResolveActionInputsService;

impl ResolveActionInputsService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ResolveActionInputsService {
    fn default() -> Self {
        Self::new()
    }
}

impl ResolveActionInputsPort for ResolveActionInputsService {
    fn execute(
        &self,
        request: ResolveActionInputsRequest<'_>,
    ) -> Result<HashMap<String, String>, StepError> {
        let mut inputs: HashMap<String, String> = request
            .definition()
            .inputs()
            .iter()
            .filter_map(|(name, input)| {
                input
                    .default()
                    .map(|default| (name.clone(), default.to_string()))
            })
            .collect();
        inputs.extend(
            request
                .step()
                .with()
                .iter()
                .map(|(name, value)| (name.clone(), value.clone())),
        );
        let missing = request
            .definition()
            .inputs()
            .iter()
            .find(|(name, input)| input.required() && !inputs.contains_key(*name));
        if let Some((name, _input)) = missing {
            return Err(StepError::new(format!(
                "required action input '{name}' was not supplied"
            )));
        }
        Ok(inputs)
    }
}
