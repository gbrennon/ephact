use super::{deserialize, serialize};
use crate::domain::value_objects::{Marker, MarkerPreset};

#[test]
fn serializes_and_deserializes_preset_marker() {
    let marker = Marker::preset(MarkerPreset::Chevron);
    assert_eq!(deserialize(&serialize(&marker)), marker);
}

#[test]
fn serializes_and_deserializes_custom_marker() {
    let marker = Marker::custom_text("🚀");
    assert_eq!(deserialize(&serialize(&marker)), marker);
}

#[test]
fn serializes_and_deserializes_image_marker() {
    let marker = Marker::image_path("/tmp/marker.png");
    assert_eq!(deserialize(&serialize(&marker)), marker);
}

#[test]
fn serializes_and_deserializes_gif_marker() {
    let marker = Marker::gif_path("/tmp/marker.gif");
    assert_eq!(deserialize(&serialize(&marker)), marker);
}

#[test]
fn unknown_marker_values_become_custom_text() {
    assert_eq!(deserialize("unknown"), Marker::custom_text("unknown"));
}
