use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use super::super::theme::Theme;
use crate::{
    application::ports::outbound::SettingsStorePort,
    domain::{InterfaceMode, Marker, MarkerKind, MarkerPreset, Settings},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsAction {
    Continue,
    Back,
    Saved,
}

#[derive(Clone)]
pub struct SettingsScreen {
    settings: Settings,
    edit_backup: Settings,
    store: Option<Arc<dyn SettingsStorePort>>,
    selected_index: usize,
    editing: bool,
    error: Option<String>,
    marker_custom_editing: bool,
}

impl SettingsScreen {
    const SETTING_COUNT: usize = 10;
    const MARKER_INDEX: usize = 9;
    const CUSTOM_MARKER_INDEX: usize = MarkerPreset::ALL.len();
    const TITLE: &'static str = "Settings";
    const CONTENT_MIN_HEIGHT: u16 = 5;
    const FOOTER_HEIGHT: u16 = 1;
    pub fn new(settings: Settings, store: Option<Arc<dyn SettingsStorePort>>) -> Self {
        Self {
            edit_backup: settings.clone(),
            settings,
            store,
            selected_index: 0,
            editing: false,
            error: None,
            marker_custom_editing: false,
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> SettingsAction {
        if self.editing {
            return self.handle_editing_key(key);
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_index = self.selected_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_index = (self.selected_index + 1).min(Self::SETTING_COUNT - 1);
            }
            KeyCode::Enter => {
                self.edit_backup = self.settings.clone();
                self.editing = true;
                self.marker_custom_editing = self.selected_index == Self::MARKER_INDEX
                    && self.settings.marker().kind() == MarkerKind::CustomText;
            }
            KeyCode::Char('s') => return self.save(),
            KeyCode::Esc | KeyCode::Backspace => return SettingsAction::Back,
            _ => {}
        }
        SettingsAction::Continue
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .padding(Padding::horizontal(1))
            .title(Span::styled(Self::TITLE, Theme::title_style()));
        let content_area = block.inner(area);
        frame.render_widget(block, area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(Self::CONTENT_MIN_HEIGHT),
                Constraint::Length(Self::FOOTER_HEIGHT),
            ])
            .split(content_area);
        self.render_settings(frame, chunks[0]);
        self.render_footer(frame, chunks[1]);
    }

    fn render_settings(&self, frame: &mut Frame<'_>, area: Rect) {
        let items = self
            .setting_lines()
            .into_iter()
            .map(ListItem::new)
            .collect::<Vec<_>>();
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

    fn render_footer(&self, frame: &mut Frame<'_>, area: Rect) {
        let footer = Paragraph::new(self.footer()).style(Theme::muted_style());
        frame.render_widget(footer, area);
    }

    pub fn footer(&self) -> &'static str {
        if self.editing && self.selected_index == Self::MARKER_INDEX {
            if self.marker_custom_editing {
                "Type: Custom | Bksp: Delete | Enter: Confirm | Esc: Cancel | q: Quit"
            } else {
                "Left/Right: Choose | Enter: Confirm | Esc: Cancel | q: Quit"
            }
        } else if self.editing {
            "Left/Right: Choose | Enter: Confirm | Esc: Cancel | q: Quit"
        } else {
            "Up/Down/j/k: Move | Enter: Edit | s: Save | Esc/Bksp: Back | q: Quit"
        }
    }

    fn handle_editing_key(&mut self, key: KeyEvent) -> SettingsAction {
        if self.selected_index == Self::MARKER_INDEX {
            return self.handle_marker_key(key);
        }
        match key.code {
            KeyCode::Enter => self.editing = false,
            KeyCode::Esc => {
                self.settings = self.edit_backup.clone();
                self.editing = false;
            }
            KeyCode::Left | KeyCode::Right => self.toggle_selected(),
            _ => {}
        }
        SettingsAction::Continue
    }

    fn handle_marker_key(&mut self, key: KeyEvent) -> SettingsAction {
        if self.marker_custom_editing {
            return self.handle_custom_marker_key(key);
        }
        self.handle_marker_navigation_key(key)
    }

    fn handle_custom_marker_key(&mut self, key: KeyEvent) -> SettingsAction {
        if key.code == KeyCode::Backspace {
            self.delete_custom_marker_character();
            return SettingsAction::Continue;
        }
        if let KeyCode::Char(character) = key.code {
            self.append_custom_marker_character(character);
            return SettingsAction::Continue;
        }
        self.handle_marker_navigation_key(key)
    }

    fn handle_marker_navigation_key(&mut self, key: KeyEvent) -> SettingsAction {
        match key.code {
            KeyCode::Enter => {
                self.editing = false;
                self.marker_custom_editing = false;
            }
            KeyCode::Esc => {
                self.settings = self.edit_backup.clone();
                self.editing = false;
                self.marker_custom_editing = false;
            }
            KeyCode::Left => self.select_previous_marker(),
            KeyCode::Right => self.select_next_marker(),
            _ => {}
        }
        SettingsAction::Continue
    }

    fn delete_custom_marker_character(&mut self) {
        if let Marker::CustomText(value) = self.settings.marker() {
            let mut value = value.clone();
            value.pop();
            self.settings = self
                .settings
                .clone()
                .with_marker(Marker::custom_text(value));
        }
    }

    fn append_custom_marker_character(&mut self, character: char) {
        if let Marker::CustomText(value) = self.settings.marker() {
            let mut value = value.clone();
            value.push(character);
            self.settings = self
                .settings
                .clone()
                .with_marker(Marker::custom_text(value));
        }
    }

    fn select_previous_marker(&mut self) {
        let index = self.marker_index();
        if index == 0 {
            return;
        }
        self.choose_marker_option(index - 1);
    }

    fn select_next_marker(&mut self) {
        let index = self.marker_index();
        if index < Self::CUSTOM_MARKER_INDEX {
            self.choose_marker_option(index + 1);
        } else {
            self.marker_custom_editing = true;
            if !matches!(self.settings.marker(), Marker::CustomText(_)) {
                self.settings = self.settings.clone().with_marker(Marker::custom_text(""));
            }
        }
    }

    fn marker_index(&self) -> usize {
        self.settings
            .marker()
            .preset_value()
            .and_then(|preset| MarkerPreset::ALL.iter().position(|item| *item == preset))
            .unwrap_or(Self::CUSTOM_MARKER_INDEX)
    }

    fn choose_marker_option(&mut self, index: usize) {
        if let Some(preset) = MarkerPreset::ALL.get(index).copied() {
            self.settings = self.settings.clone().with_marker(Marker::preset(preset));
            self.marker_custom_editing = false;
        } else {
            self.marker_custom_editing = true;
            if !matches!(self.settings.marker(), Marker::CustomText(_)) {
                self.settings = self.settings.clone().with_marker(Marker::custom_text(""));
            }
        }
    }

    fn save(&mut self) -> SettingsAction {
        let Some(store) = self.store.as_ref() else {
            self.error = Some("settings store is not configured".to_string());
            return SettingsAction::Continue;
        };
        match store.write_settings(&self.settings) {
            Ok(()) => {
                self.error = None;
                SettingsAction::Saved
            }
            Err(error) => {
                self.error = Some(error.to_string());
                SettingsAction::Continue
            }
        }
    }

    fn toggle_selected(&mut self) {
        let value = match self.selected_index {
            0 => self.settings.default_interface() == InterfaceMode::Tui,
            1 => self.settings.allow_repo_writes(),
            2 => self.settings.allow_real_container(),
            3 => self.settings.allow_real_fetcher(),
            4 => self.settings.allow_network(),
            5 => self.settings.preserve(),
            6 => self.settings.verbose(),
            7 => self.settings.interactive(),
            8 => self.settings.all_workflows(),
            _ => return,
        };
        self.settings = match self.selected_index {
            0 => self.settings.clone().with_default_interface(if value {
                InterfaceMode::Cli
            } else {
                InterfaceMode::Tui
            }),
            1 => self.settings.clone().with_allow_repo_writes(!value),
            2 => self.settings.clone().with_allow_real_container(!value),
            3 => self.settings.clone().with_allow_real_fetcher(!value),
            4 => self.settings.clone().with_allow_network(!value),
            5 => self.settings.clone().with_preserve(!value),
            6 => self.settings.clone().with_verbose(!value),
            7 => self.settings.clone().with_interactive(!value),
            8 => self.settings.clone().with_all_workflows(!value),
            _ => self.settings.clone(),
        };
    }

    fn setting_lines(&self) -> Vec<Line<'static>> {
        self.setting_lines_primary()
            .into_iter()
            .chain(self.setting_lines_secondary())
            .chain(std::iter::once(self.marker_line()))
            .collect()
    }

    fn marker_line(&self) -> Line<'static> {
        if !self.editing || self.selected_index != Self::MARKER_INDEX {
            return Line::from(format!("marker = {}", self.settings.marker().as_text()));
        }
        if self.marker_custom_editing {
            return self.custom_marker_line();
        }
        self.preset_marker_line()
    }

    fn custom_marker_line(&self) -> Line<'static> {
        let value = self.settings.marker().as_text();
        Line::from(vec![
            Span::raw("marker = Custom: "),
            Span::styled(value.to_string(), Theme::body_style()),
            Span::styled("|", Theme::selection_style()),
        ])
    }

    fn preset_marker_line(&self) -> Line<'static> {
        let selected = self.marker_index();
        let spans = MarkerPreset::ALL
            .iter()
            .enumerate()
            .map(|(index, preset)| Self::preset_marker_span(index, *preset, selected))
            .chain(std::iter::once(Self::custom_marker_span(selected)))
            .collect::<Vec<_>>();
        Line::from(
            std::iter::once(Span::raw("marker = "))
                .chain(spans.into_iter().flat_map(|span| [span, Span::raw(" ")]))
                .collect::<Vec<_>>(),
        )
    }

    fn preset_marker_span(index: usize, preset: MarkerPreset, selected: usize) -> Span<'static> {
        let is_selected = index == selected;
        let text = if is_selected {
            format!("[{}]", preset.as_text())
        } else {
            preset.as_text().to_string()
        };
        Span::styled(text, Self::marker_option_style(is_selected))
    }

    fn custom_marker_span(selected: usize) -> Span<'static> {
        let is_selected = selected == Self::CUSTOM_MARKER_INDEX;
        let text = if is_selected { "[Custom]" } else { "Custom" };
        Span::styled(text, Self::marker_option_style(is_selected))
    }

    fn marker_option_style(selected: bool) -> Style {
        if selected {
            Theme::selection_style()
        } else {
            Theme::body_style()
        }
    }

    fn setting_lines_primary(&self) -> Vec<Line<'static>> {
        vec![
            self.option_line(
                0,
                "default-interface",
                interface_name(self.settings.default_interface()),
                &["tui", "cli"],
            ),
            self.option_line(
                1,
                "allow-repo-writes",
                bool_name(self.settings.allow_repo_writes()),
                &["false", "true"],
            ),
            self.option_line(
                2,
                "allow-real-container",
                bool_name(self.settings.allow_real_container()),
                &["false", "true"],
            ),
            self.option_line(
                3,
                "allow-real-fetcher",
                bool_name(self.settings.allow_real_fetcher()),
                &["false", "true"],
            ),
        ]
    }

    fn setting_lines_secondary(&self) -> Vec<Line<'static>> {
        vec![
            self.option_line(
                4,
                "allow-network",
                bool_name(self.settings.allow_network()),
                &["false", "true"],
            ),
            self.option_line(
                5,
                "preserve",
                bool_name(self.settings.preserve()),
                &["false", "true"],
            ),
            self.option_line(
                6,
                "verbose",
                bool_name(self.settings.verbose()),
                &["false", "true"],
            ),
            self.option_line(
                7,
                "interactive",
                bool_name(self.settings.interactive()),
                &["false", "true"],
            ),
            self.option_line(
                8,
                "all-workflows",
                bool_name(self.settings.all_workflows()),
                &["false", "true"],
            ),
        ]
    }

    fn option_line(
        &self,
        index: usize,
        name: &'static str,
        value: &'static str,
        options: &'static [&'static str],
    ) -> Line<'static> {
        if !self.editing || self.selected_index != index {
            return Line::from(format!("{name} = {value}"));
        }
        let spans = options
            .iter()
            .map(|option| {
                let selected = *option == value;
                let style = if selected {
                    Theme::selection_style()
                } else {
                    Theme::body_style()
                };
                let text = if selected {
                    format!("[{option}]")
                } else {
                    (*option).to_string()
                };
                Span::styled(text, style)
            })
            .collect::<Vec<_>>();
        Line::from(
            std::iter::once(Span::raw(format!("{name} = ")))
                .chain(spans.into_iter().flat_map(|span| [span, Span::raw(" ")]))
                .collect::<Vec<_>>(),
        )
    }
}

fn bool_name(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
fn interface_name(mode: InterfaceMode) -> &'static str {
    match mode {
        InterfaceMode::Tui => "tui",
        InterfaceMode::Cli => "cli",
    }
}
