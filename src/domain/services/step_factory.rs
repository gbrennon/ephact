use crate::domain::entities::Step;
use crate::domain::errors::StepError;

pub struct StepFactory;

impl StepFactory {
    pub fn to_text(step: &Step) -> Result<String, StepError> {
        serde_json::to_string(step)
            .map_err(|error| StepError::new(format!("failed to encode step: {error}")))
    }

    pub fn from_text(text: &str) -> Result<Step, StepError> {
        serde_json::from_str(text)
            .map_err(|error| StepError::new(format!("failed to decode step: {error}")))
    }
}

impl From<Step> for String {
    fn from(step: Step) -> Self {
        StepFactory::to_text(&step).expect("steps contain only JSON-compatible values")
    }
}
