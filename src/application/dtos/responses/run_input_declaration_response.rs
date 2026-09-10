use crate::application::dtos::responses::RunInputSourceResponse;

/// Input metadata returned when a workflow run exposes configurable inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunInputDeclarationResponse {
    name: String,
    source: RunInputSourceResponse,
    description: Option<String>,
    required: bool,
    default: Option<String>,
    resolved: bool,
}

impl RunInputDeclarationResponse {
    /// Creates a response for one declared input.
    pub fn new(
        name: impl Into<String>,
        source: RunInputSourceResponse,
        description: Option<String>,
        required: bool,
        default: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            source,
            description,
            required,
            default,
            resolved: false,
        }
    }

    /// Returns a response with the resolved flag updated.
    pub fn with_resolved(mut self, resolved: bool) -> Self {
        self.resolved = resolved;
        self
    }

    /// Returns the input name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns where the input was declared.
    pub fn source(&self) -> &RunInputSourceResponse {
        &self.source
    }

    /// Returns human-readable input guidance when one was declared.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns whether the input must be provided by the caller.
    pub fn required(&self) -> bool {
        self.required
    }

    /// Returns the default value declared for the input.
    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Returns whether the input is already resolved for execution.
    pub fn is_resolved(&self) -> bool {
        self.resolved || self.default.is_some()
    }
}
