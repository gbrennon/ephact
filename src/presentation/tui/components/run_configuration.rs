use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use crate::{
    application::dtos::responses::RunInputDeclarationResponse,
    domain::value_objects::Marker,
    presentation::tui::{
        components::run_configuration_input::{InputField, InputKind},
        theme::Theme,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunConfigurationValues {
    event: String,
    inputs: Vec<(String, String)>,
}

impl RunConfigurationValues {
    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn inputs(&self) -> &[(String, String)] {
        &self.inputs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationAction {
    Continue,
    Submit,
    Cancel,
}

#[derive(Clone)]
pub struct RunConfiguration {
    marker: Marker,
    events: Vec<String>,
    selected_index: usize,
    selected_event: usize,
    inputs: Vec<InputField>,
    editing: bool,
    error: Option<String>,
}

impl RunConfiguration {
    const EVENT_LABEL: &'static str = "Event: ";
    const ERROR_LABEL: &'static str = "Error: ";
    const EVENT_PICKER_FOOTER: &'static str =
        "Up/Down/j/k: Select event | Enter: Select | Esc/Bksp: Back | q: Quit";
    const INPUT_PICKER_FOOTER: &'static str =
        "Up/Down/j/k: Move | Enter: Edit | r: Run | Esc/Bksp: Back | q: Quit";
    const RUN_FOOTER: &'static str = "Up/Down/j/k: Move | r: Run | Esc/Bksp: Back | q: Quit";

    pub fn new(events: Vec<String>, declarations: Vec<RunInputDeclarationResponse>) -> Self {
        Self {
            marker: Marker::default(),
            events,
            selected_index: 0,
            selected_event: 0,
            inputs: declarations
                .iter()
                .map(InputField::from_declaration)
                .collect(),
            editing: false,
            error: None,
        }
    }

    pub fn with_marker(mut self, marker: &Marker) -> Self {
        self.marker = marker.clone();
        self
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ConfigurationAction {
        if self.error.is_some() {
            return self.handle_error_key(key);
        }
        if self.editing {
            return self.handle_editing_key(key);
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Enter => {
                if self.is_selecting_event() {
                    return self.submit();
                }
                self.start_editing();
            }
            KeyCode::Char('r') if !self.inputs.is_empty() || self.events.is_empty() => {
                return self.submit();
            }
            KeyCode::Esc | KeyCode::Backspace => return ConfigurationAction::Cancel,
            _ => {}
        }
        ConfigurationAction::Continue
    }

    pub fn values(&self) -> Option<RunConfigurationValues> {
        let event = self.events.get(self.selected_event)?.clone();
        Some(RunConfigurationValues {
            event,
            inputs: self
                .inputs
                .iter()
                .map(|input| (input.name().to_string(), input.value().to_string()))
                .collect(),
        })
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        if let Some(error) = self.error.as_deref() {
            self.render_error_modal(frame, area, error);
            return;
        }
        let mut items = self
            .events
            .iter()
            .enumerate()
            .map(|(index, event)| self.event_item(index, event))
            .chain(
                self.inputs
                    .iter()
                    .enumerate()
                    .map(|(index, input)| self.input_item(index + self.events.len(), input)),
            )
            .collect::<Vec<_>>();
        if let Some(error) = self.error.as_deref() {
            items.push(ListItem::new(Line::from(Span::styled(
                error,
                Theme::critical_style(),
            ))));
        }
        let mut state = ListState::default();
        state.select(Some(self.selected_index));
        frame.render_stateful_widget(
            List::new(items)
                .style(Theme::body_style())
                .highlight_style(Theme::selection_style()),
            area,
            &mut state,
        );
    }

    pub fn footer(&self) -> &'static str {
        if self.error.is_some() {
            "Enter/Esc/Bksp: Dismiss | q: Quit"
        } else if self.editing {
            self.editing_footer()
        } else if self.is_selecting_event() {
            Self::EVENT_PICKER_FOOTER
        } else if self.selected_index >= self.events.len() {
            Self::INPUT_PICKER_FOOTER
        } else {
            Self::RUN_FOOTER
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    fn editing_footer(&self) -> &'static str {
        match self.current_input().kind() {
            InputKind::Boolean => "Left/h: False | Right/l: True | Enter/Esc: Finish",
            InputKind::Text => "Type: Edit | Enter/Esc: Finish | Bksp: Delete",
        }
    }

    fn is_selecting_event(&self) -> bool {
        self.inputs.is_empty() && self.selected_index < self.events.len()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn report_error(&mut self, error: String) {
        self.error = Some(error);
    }

    fn handle_error_key(&mut self, key: KeyEvent) -> ConfigurationAction {
        if matches!(key.code, KeyCode::Enter | KeyCode::Esc | KeyCode::Backspace) {
            self.error = None;
            return ConfigurationAction::Cancel;
        }
        ConfigurationAction::Continue
    }

    fn render_error_modal(&self, frame: &mut Frame<'_>, area: Rect, error: &str) {
        let width = area.width.min(60);
        let height = area.height.min(5);
        let x = area.x + area.width.saturating_sub(width) / 2;
        let y = area.y + area.height.saturating_sub(height) / 2;
        let modal = Rect::new(x, y, width, height);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::critical_style())
            .title(Span::styled("Cannot Run", Theme::title_style()));
        frame.render_widget(Clear, modal);
        frame.render_widget(
            Paragraph::new(error)
                .alignment(Alignment::Center)
                .block(block),
            modal,
        );
    }

    fn handle_editing_key(&mut self, key: KeyEvent) -> ConfigurationAction {
        match key.code {
            KeyCode::Enter | KeyCode::Esc => self.editing = false,
            _ => self.current_input_mut().handle_key(key.code),
        }
        ConfigurationAction::Continue
    }

    fn start_editing(&mut self) {
        let input_count = self.inputs.len();
        if self.selected_index >= self.events.len()
            && self.selected_index < self.events.len() + input_count
        {
            self.editing = true;
        }
    }

    fn submit(&mut self) -> ConfigurationAction {
        if self.events.is_empty() {
            self.error = Some("Error: Workflow declares no supported events".to_owned());
            return ConfigurationAction::Continue;
        }
        if let Some(input) = self
            .inputs
            .iter()
            .find(|input| input.required() && input.value().is_empty())
        {
            self.error = Some(format!("{}{} is required", Self::ERROR_LABEL, input.name()));
            return ConfigurationAction::Continue;
        }
        self.error = None;
        ConfigurationAction::Submit
    }

    fn select_next(&mut self) {
        let item_count = self.events.len() + self.inputs.len();
        if item_count == 0 {
            return;
        }
        let last_index = item_count - 1;
        self.selected_index = (self.selected_index + 1).min(last_index);
        if self.selected_index < self.events.len() {
            self.selected_event = self.selected_index;
        }
    }

    fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
        if self.selected_index < self.events.len() {
            self.selected_event = self.selected_index;
        }
    }

    fn current_input(&self) -> &InputField {
        &self.inputs[self.selected_index - self.events.len()]
    }

    fn current_input_mut(&mut self) -> &mut InputField {
        &mut self.inputs[self.selected_index - self.events.len()]
    }

    fn event_item(&self, _index: usize, event: &str) -> ListItem<'static> {
        ListItem::new(Line::from(Span::styled(
            format!("{}{}", Self::EVENT_LABEL, event),
            Theme::body_style(),
        )))
    }

    fn input_item(&self, index: usize, input: &InputField) -> ListItem<'static> {
        input.render(index, self.selected_index, self.editing, &self.marker)
    }
}
