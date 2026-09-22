use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};

use crate::{
    application::dtos::responses::RunInputDeclarationResponse, presentation::tui::theme::Theme,
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

struct InputField {
    name: String,
    required: bool,
    value: String,
}

pub struct RunConfiguration {
    events: Vec<String>,
    selected_index: usize,
    selected_event: usize,
    inputs: Vec<InputField>,
    editing: bool,
    error: Option<String>,
}

impl RunConfiguration {
    const DEFAULT_EVENT: &'static str = "pull_request";
    const EVENT_LABEL: &'static str = "Event: ";
    const INPUT_LABEL: &'static str = "Input: ";
    const ERROR_LABEL: &'static str = "Error: ";
    const REQUIRED_SUFFIX: &'static str = " (required)";

    pub fn new(events: Vec<String>, declarations: Vec<RunInputDeclarationResponse>) -> Self {
        let events = if events.is_empty() {
            vec![Self::DEFAULT_EVENT.to_string()]
        } else {
            events
        };
        Self {
            events,
            selected_index: 0,
            selected_event: 0,
            inputs: declarations
                .into_iter()
                .map(|declaration| InputField {
                    name: declaration.name().to_string(),
                    required: declaration.required() && declaration.default().is_none(),
                    value: declaration.default().unwrap_or_default().to_string(),
                })
                .collect(),
            editing: false,
            error: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ConfigurationAction {
        if self.editing {
            return self.handle_editing_key(key);
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Enter => self.start_editing(),
            KeyCode::Char('r') => return self.submit(),
            KeyCode::Esc | KeyCode::Backspace => return ConfigurationAction::Cancel,
            _ => {}
        }
        ConfigurationAction::Continue
    }

    pub fn values(&self) -> RunConfigurationValues {
        RunConfigurationValues {
            event: self.events[self.selected_event].clone(),
            inputs: self
                .inputs
                .iter()
                .map(|input| (input.name.clone(), input.value.clone()))
                .collect(),
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        let mut items = self
            .events
            .iter()
            .enumerate()
            .map(|(index, event)| self.event_item(index, event))
            .chain(self.inputs.iter().map(|input| self.input_item(input)))
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
        "Up/Down: Select | Enter: Edit input | r: Run | Esc: Back"
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn report_error(&mut self, error: String) {
        self.error = Some(error);
    }

    fn handle_editing_key(&mut self, key: KeyEvent) -> ConfigurationAction {
        match key.code {
            KeyCode::Enter => self.editing = false,
            KeyCode::Esc => self.editing = false,
            KeyCode::Backspace => {
                self.current_input_mut().value.pop();
            }
            KeyCode::Char(character) => self.current_input_mut().value.push(character),
            _ => {}
        }
        ConfigurationAction::Continue
    }

    fn start_editing(&mut self) {
        if self.selected_index >= self.events.len() {
            self.editing = true;
        }
    }

    fn submit(&mut self) -> ConfigurationAction {
        if let Some(input) = self
            .inputs
            .iter()
            .find(|input| input.required && input.value.is_empty())
        {
            self.error = Some(format!("{}{} is required", Self::ERROR_LABEL, input.name));
            return ConfigurationAction::Continue;
        }
        self.error = None;
        ConfigurationAction::Submit
    }

    fn select_next(&mut self) {
        let last_index = self.events.len() + self.inputs.len() - 1;
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

    fn current_input_mut(&mut self) -> &mut InputField {
        &mut self.inputs[self.selected_index - self.events.len()]
    }

    fn event_item(&self, _index: usize, event: &str) -> ListItem<'static> {
        ListItem::new(Line::from(Span::styled(
            format!("{}{}", Self::EVENT_LABEL, event),
            Theme::body_style(),
        )))
    }

    fn input_item(&self, input: &InputField) -> ListItem<'static> {
        let required = if input.required {
            Self::REQUIRED_SUFFIX
        } else {
            ""
        };
        ListItem::new(Line::from(format!(
            "{}{}{} = {}",
            Self::INPUT_LABEL,
            input.name,
            required,
            input.value
        )))
    }
}
