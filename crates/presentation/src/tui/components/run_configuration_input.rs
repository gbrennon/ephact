use crossterm::event::KeyCode;
use ratatui::{
    text::{Line, Span},
    widgets::ListItem,
};

use crate::{
    application::dtos::responses::RunInputDeclarationResponse, domain::value_objects::Marker,
    tui::theme::Theme,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum InputKind {
    Text,
    Boolean,
}

impl InputKind {
    fn from_declaration(input_type: Option<&str>, default_value: Option<&str>) -> Self {
        match input_type {
            Some("bool") | Some("boolean") => Self::Boolean,
            _ if matches!(default_value, Some("true" | "false")) => Self::Boolean,
            _ => Self::Text,
        }
    }
}

#[derive(Clone)]
pub(super) struct InputField {
    name: String,
    required: bool,
    value: String,
    kind: InputKind,
}

impl InputField {
    pub(super) fn from_declaration(declaration: &RunInputDeclarationResponse) -> Self {
        let kind = InputKind::from_declaration(declaration.input_type(), declaration.default());
        let value = match kind {
            InputKind::Text => declaration.default().unwrap_or_default().to_string(),
            InputKind::Boolean => declaration.default().unwrap_or("false").to_string(),
        };
        Self {
            name: declaration.name().to_string(),
            required: declaration.required()
                && declaration.default().is_none()
                && kind == InputKind::Text,
            value,
            kind,
        }
    }

    pub(super) fn kind(&self) -> InputKind {
        self.kind
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn required(&self) -> bool {
        self.required
    }

    pub(super) fn value(&self) -> &str {
        &self.value
    }

    pub(super) fn handle_key(&mut self, key_code: KeyCode) {
        match (self.kind, key_code) {
            (InputKind::Boolean, KeyCode::Left | KeyCode::Down | KeyCode::Char('h')) => {
                self.value = "false".to_string();
            }
            (InputKind::Boolean, KeyCode::Right | KeyCode::Up | KeyCode::Char('l')) => {
                self.value = "true".to_string();
            }
            (InputKind::Text, KeyCode::Backspace) => {
                self.value.pop();
            }
            (InputKind::Text, KeyCode::Char(character)) => self.value.push(character),
            _ => {}
        }
    }

    pub(super) fn render(
        &self,
        index: usize,
        selected_index: usize,
        editing: bool,
        marker: &Marker,
    ) -> ListItem<'static> {
        let required = if self.required { " (required)" } else { "" };
        let line = if editing && selected_index == index {
            self.editing_line(required, marker)
        } else {
            Line::from(format!("Input: {}{} = {}", self.name, required, self.value))
        };
        ListItem::new(line)
    }

    fn editing_line(&self, required: &str, marker: &Marker) -> Line<'static> {
        if self.kind != InputKind::Boolean {
            return self.text_editing_line(required, marker);
        }
        self.boolean_editing_line(required)
    }

    fn text_editing_line(&self, required: &str, marker: &Marker) -> Line<'static> {
        Line::from(vec![
            Span::raw(format!("Input: {}{} = {}", self.name, required, self.value)),
            Span::styled(marker.as_text().to_string(), Theme::selection_style()),
        ])
    }

    fn boolean_editing_line(&self, required: &str) -> Line<'static> {
        Line::from(vec![
            Span::raw(format!("Input: {}{} = ", self.name, required)),
            Self::boolean_option_span(&self.value, "false"),
            Span::raw(" "),
            Self::boolean_option_span(&self.value, "true"),
        ])
    }

    fn boolean_option_span(value: &str, option: &str) -> Span<'static> {
        let selected = value == option;
        let text = if selected {
            format!("[{option}]")
        } else {
            option.to_string()
        };
        let style = if selected {
            Theme::selection_style()
        } else {
            Theme::body_style()
        };
        Span::styled(text, style)
    }
}
