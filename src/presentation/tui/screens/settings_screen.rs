use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use super::super::theme::Theme;
use crate::{
    application::ports::outbound::SettingsStorePort,
    domain::{InterfaceMode, Settings},
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
}

impl SettingsScreen {
    const SETTING_COUNT: usize = 9;
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
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
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
        "Up/Down: Select | Enter: Edit | Left/Right: Choose | s: Save | Esc: Cancel"
    }

    fn handle_editing_key(&mut self, key: KeyEvent) -> SettingsAction {
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
            .collect()
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
