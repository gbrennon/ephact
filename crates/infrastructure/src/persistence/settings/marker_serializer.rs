use crate::domain::value_objects::{Marker, MarkerPreset};

pub(super) fn default_marker() -> String {
    Marker::default().value().to_owned()
}

pub(super) fn serialize(marker: &Marker) -> String {
    match marker {
        Marker::Preset(preset) => preset.as_text().to_owned(),
        Marker::CustomText(value) => format!("text:{value}"),
        Marker::ImagePath(value) => format!("image:{value}"),
        Marker::GifPath(value) => format!("gif:{value}"),
    }
}

pub(super) fn deserialize(value: &str) -> Marker {
    if let Some(value) = value.strip_prefix("text:") {
        return Marker::custom_text(value);
    }
    if let Some(value) = value.strip_prefix("image:") {
        return Marker::image_path(value);
    }
    if let Some(value) = value.strip_prefix("gif:") {
        return Marker::gif_path(value);
    }
    if let Some(preset) = MarkerPreset::ALL
        .into_iter()
        .find(|preset| preset.as_text() == value)
    {
        return Marker::preset(preset);
    }
    Marker::custom_text(value)
}

#[cfg(test)]
mod marker_serializer_tests;
