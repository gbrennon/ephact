use crate::{application::dtos::responses::StepSummaryDetails, domain::value_objects::StepType};

pub struct StepSummaryResponseInput {
    name: String,
    step_type: StepType,
    details: StepSummaryDetails,
}

impl StepSummaryResponseInput {
    pub fn new(name: impl Into<String>, step_type: StepType, details: StepSummaryDetails) -> Self {
        Self {
            name: name.into(),
            step_type,
            details,
        }
    }

    pub(crate) fn into_parts(self) -> (String, StepType, StepSummaryDetails) {
        (self.name, self.step_type, self.details)
    }
}
