#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BuiltinFunction {
    Contains,
    StartsWith,
    EndsWith,
    Format,
    Join,
    ToJson,
    FromJson,
    Success,
    Always,
    Cancelled,
    Failure,
}

impl BuiltinFunction {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "contains" => Some(Self::Contains),
            "startswith" => Some(Self::StartsWith),
            "endswith" => Some(Self::EndsWith),
            "format" => Some(Self::Format),
            "join" => Some(Self::Join),
            "tojson" => Some(Self::ToJson),
            "fromjson" => Some(Self::FromJson),
            "success" => Some(Self::Success),
            "always" => Some(Self::Always),
            "cancelled" => Some(Self::Cancelled),
            "failure" => Some(Self::Failure),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Contains => "contains",
            Self::StartsWith => "startswith",
            Self::EndsWith => "endswith",
            Self::Format => "format",
            Self::Join => "join",
            Self::ToJson => "tojson",
            Self::FromJson => "fromjson",
            Self::Success => "success",
            Self::Always => "always",
            Self::Cancelled => "cancelled",
            Self::Failure => "failure",
        }
    }

    pub fn required_arguments(&self) -> usize {
        match self {
            Self::Contains | Self::StartsWith | Self::EndsWith | Self::Join => 2,
            Self::Format => 1,
            Self::ToJson | Self::FromJson => 1,
            Self::Success | Self::Always | Self::Cancelled | Self::Failure => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn represents_every_supported_name_and_required_arity() {
        let cases = [
            ("contains", 2),
            ("startsWith", 2),
            ("endsWith", 2),
            ("format", 1),
            ("join", 2),
            ("toJson", 1),
            ("fromJson", 1),
            ("success", 0),
            ("always", 0),
            ("cancelled", 0),
            ("failure", 0),
        ];

        for (name, required_arguments) in cases {
            let function = BuiltinFunction::from_name(name).unwrap();
            assert_eq!(function.name(), name.to_ascii_lowercase());
            assert_eq!(function.required_arguments(), required_arguments);
        }
    }

    #[test]
    fn name_lookup_is_case_insensitive_and_rejects_unknown_names() {
        assert_eq!(
            BuiltinFunction::from_name("CoNtAiNs"),
            Some(BuiltinFunction::Contains)
        );
        assert_eq!(BuiltinFunction::from_name("unknown"), None);
    }
}
