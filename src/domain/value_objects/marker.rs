#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerPreset {
    GreaterThan,
    Underscore,
    Dollar,
    Hash,
    Chevron,
    Arrow,
    Play,
}

impl MarkerPreset {
    pub const ALL: [Self; 7] = [
        Self::GreaterThan,
        Self::Underscore,
        Self::Dollar,
        Self::Hash,
        Self::Chevron,
        Self::Arrow,
        Self::Play,
    ];

    pub fn as_text(self) -> &'static str {
        match self {
            Self::GreaterThan => ">",
            Self::Underscore => "_",
            Self::Dollar => "$",
            Self::Hash => "#",
            Self::Chevron => "❯",
            Self::Arrow => "➜",
            Self::Play => "▶",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerKind {
    Preset,
    CustomText,
    ImagePath,
    GifPath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Marker {
    Preset(MarkerPreset),
    CustomText(String),
    ImagePath(String),
    GifPath(String),
}

impl Default for Marker {
    fn default() -> Self {
        Self::Preset(MarkerPreset::Underscore)
    }
}

impl Marker {
    pub fn preset(preset: MarkerPreset) -> Self {
        Self::Preset(preset)
    }

    pub fn custom_text(value: impl Into<String>) -> Self {
        Self::CustomText(value.into())
    }

    pub fn image_path(value: impl Into<String>) -> Self {
        Self::ImagePath(value.into())
    }

    pub fn gif_path(value: impl Into<String>) -> Self {
        Self::GifPath(value.into())
    }

    pub fn kind(&self) -> MarkerKind {
        match self {
            Self::Preset(_) => MarkerKind::Preset,
            Self::CustomText(_) => MarkerKind::CustomText,
            Self::ImagePath(_) => MarkerKind::ImagePath,
            Self::GifPath(_) => MarkerKind::GifPath,
        }
    }

    pub fn as_text(&self) -> &str {
        match self {
            Self::Preset(preset) => preset.as_text(),
            Self::CustomText(value) => value,
            Self::ImagePath(_) | Self::GifPath(_) => "_",
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Preset(preset) => preset.as_text(),
            Self::CustomText(value) | Self::ImagePath(value) | Self::GifPath(value) => value,
        }
    }

    pub fn preset_value(&self) -> Option<MarkerPreset> {
        match self {
            Self::Preset(preset) => Some(*preset),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Settings;

    #[test]
    fn settings_default_to_underscore_marker() {
        assert_eq!(Settings::default().marker().as_text(), "_");
    }

    #[test]
    fn presets_expose_terminal_marker_text() {
        let values = MarkerPreset::ALL.map(MarkerPreset::as_text);

        assert_eq!(values, [">", "_", "$", "#", "❯", "➜", "▶"]);
    }

    #[test]
    fn marker_supports_custom_unicode_text() {
        let marker = Marker::custom_text("🚀");

        assert_eq!(marker.kind(), MarkerKind::CustomText);
        assert_eq!(marker.as_text(), "🚀");
        assert_eq!(marker.value(), "🚀");
    }

    #[test]
    fn marker_supports_graphical_paths() {
        let image = Marker::image_path("/tmp/marker.png");
        let gif = Marker::gif_path("/tmp/marker.gif");

        assert_eq!(image.kind(), MarkerKind::ImagePath);
        assert_eq!(image.value(), "/tmp/marker.png");
        assert_eq!(image.as_text(), "_");
        assert_eq!(gif.kind(), MarkerKind::GifPath);
        assert_eq!(gif.value(), "/tmp/marker.gif");
        assert_eq!(gif.as_text(), "_");
    }

    #[test]
    fn marker_values_compare_by_kind_and_value() {
        assert_eq!(
            Marker::preset(MarkerPreset::Dollar),
            Marker::preset(MarkerPreset::Dollar)
        );
        assert_ne!(
            Marker::preset(MarkerPreset::Dollar),
            Marker::preset(MarkerPreset::Hash)
        );
        assert_ne!(Marker::custom_text("x"), Marker::custom_text("y"));
    }
}
