use crate::application::dtos::responses::RunInputSourceResponse;

/// Input metadata returned when a workflow run exposes configurable inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunInputDeclarationResponse {
    name: String,
    source: RunInputSourceResponse,
    description: Option<String>,
    required: bool,
    default: Option<String>,
    input_type: Option<String>,
    options: Vec<String>,
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
            input_type: None,
            options: Vec::new(),
            resolved: false,
        }
    }

    /// Returns a response with the declared input type.
    pub fn with_type(mut self, input_type: Option<String>) -> Self {
        self.input_type = input_type;
        self
    }

    /// Returns a response with the declared selectable options.
    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    /// Returns the declared input type.
    pub fn input_type(&self) -> Option<&str> {
        self.input_type.as_deref()
    }

    /// Returns the declared selectable options.
    pub fn options(&self) -> &[String] {
        &self.options
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
