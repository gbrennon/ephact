use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunInputSource {
    Workflow,
    Action(String),
}

impl fmt::Display for RunInputSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workflow => formatter.write_str("workflow"),
            Self::Action(reference) => write!(formatter, "action {reference}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunInputDeclaration {
    name: String,
    source: RunInputSource,
    description: Option<String>,
    required: bool,
    default: Option<String>,
    resolved: bool,
}

impl RunInputDeclaration {
    pub fn new(
        name: impl Into<String>,
        source: RunInputSource,
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

    pub fn with_resolved(mut self, resolved: bool) -> Self {
        self.resolved = resolved;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn source(&self) -> &RunInputSource {
        &self.source
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn required(&self) -> bool {
        self.required
    }
    pub fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }
    pub fn is_resolved(&self) -> bool {
        self.resolved || self.default.is_some()
    }
}
