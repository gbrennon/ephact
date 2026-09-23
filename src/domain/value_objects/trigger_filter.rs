use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefPattern {
    Branch(String),
    Tag(String),
    Path(String),
}

impl RefPattern {
    pub fn branch(pattern: impl Into<String>) -> Self {
        Self::Branch(pattern.into())
    }

    pub fn tag(pattern: impl Into<String>) -> Self {
        Self::Tag(pattern.into())
    }

    pub fn path(pattern: impl Into<String>) -> Self {
        Self::Path(pattern.into())
    }

    pub fn pattern(&self) -> &str {
        match self {
            Self::Branch(pattern) | Self::Tag(pattern) | Self::Path(pattern) => pattern,
        }
    }
}

impl fmt::Display for RefPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Branch(pattern) => write!(formatter, "branch:{pattern}"),
            Self::Tag(pattern) => write!(formatter, "tag:{pattern}"),
            Self::Path(pattern) => write!(formatter, "path:{pattern}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TriggerFilter {
    included_refs: Vec<RefPattern>,
    excluded_refs: Vec<RefPattern>,
    event_types: Vec<String>,
}

impl TriggerFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_included_ref(mut self, reference: RefPattern) -> Self {
        self.included_refs.push(reference);
        self
    }

    pub fn with_excluded_ref(mut self, reference: RefPattern) -> Self {
        self.excluded_refs.push(reference);
        self
    }

    pub fn with_event_types(mut self, event_types: Vec<String>) -> Self {
        self.event_types = event_types;
        self
    }

    pub fn included_refs(&self) -> &[RefPattern] {
        &self.included_refs
    }

    pub fn excluded_refs(&self) -> &[RefPattern] {
        &self.excluded_refs
    }

    pub fn event_types(&self) -> &[String] {
        &self.event_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_included_and_excluded_ref_patterns() {
        let filter = TriggerFilter::new()
            .with_included_ref(RefPattern::branch("main"))
            .with_included_ref(RefPattern::tag("v*"))
            .with_excluded_ref(RefPattern::path("docs/**"));

        assert_eq!(filter.included_refs().len(), 2);
        assert_eq!(filter.included_refs()[0].pattern(), "main");
        assert_eq!(filter.excluded_refs(), &[RefPattern::path("docs/**")]);
    }

    #[test]
    fn stores_generic_event_types() {
        let filter = TriggerFilter::new().with_event_types(vec!["opened".into()]);

        assert_eq!(filter.event_types(), &["opened".to_string()]);
    }
}
