use serde::Deserialize;

/// An input parameter for `workflow_dispatch` events.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WorkflowDispatchInput {
    description: Option<String>,

    #[serde(default)]
    required: bool,

    #[serde(default)]
    default: Option<String>,

    #[serde(rename = "type")]
    #[serde(default)]
    input_type: Option<String>,

    #[serde(default)]
    options: Vec<String>,
}

impl WorkflowDispatchInput {
    pub fn new(
        description: Option<String>,
        required: bool,
        default: Option<String>,
        input_type: Option<String>,
        options: Vec<String>,
    ) -> Self {
        Self {
            description,
            required,
            default,
            input_type,
            options,
        }
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }

    pub fn input_type(&self) -> Option<&str> {
        self.input_type.as_deref()
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }
}
