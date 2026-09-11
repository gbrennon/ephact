use std::fmt;

/// Source location for an input declaration returned to callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunInputSourceResponse {
    Workflow,
    Action(String),
}

impl fmt::Display for RunInputSourceResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workflow => formatter.write_str("workflow"),
            Self::Action(reference) => write!(formatter, "action {reference}"),
        }
    }
}
