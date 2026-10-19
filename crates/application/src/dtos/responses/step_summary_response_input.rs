use super::StepSummaryDetails;
use crate::domain::value_objects::StepType;

/// Input values used to construct a step summary.
pub struct StepSummaryResponseInput {
    name: String,
    step_type: StepType,
    details: StepSummaryDetails,
}

impl StepSummaryResponseInput {
    pub fn new(name: String, step_type: StepType, details: StepSummaryDetails) -> Self {
        Self {
            name,
            step_type,
            details,
        }
    }

    pub fn into_parts(self) -> (String, StepType, StepSummaryDetails) {
        (self.name, self.step_type, self.details)
    }
}
