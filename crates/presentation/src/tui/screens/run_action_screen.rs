use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::{
    application::dtos::responses::{ExecuteActionResponse, RunInputDeclarationResponse},
    domain::value_objects::Marker,
    tui::{
        components::{ConfigurationAction, RunConfiguration, RunConfigurationValues},
        theme::Theme,
    },
};

#[derive(Clone)]
pub struct RunActionScreen {
    action_ref: String,
    configuration: Option<RunConfiguration>,
    running: bool,
    progress_lines: Vec<String>,
    outcome: Option<ExecuteActionResponse>,
    cancelled: bool,
    showing_details: bool,
    summary_scroll: u16,
}

impl RunActionScreen {
    const TITLE: &'static str = "Run Action";
    const EMPTY_FOOTER: &'static str = "Enter: Configure | Esc/Bksp: Back | q: Quit";
    const RUNNING_FOOTER: &'static str = "Esc/Bksp: Cancel";

    pub fn new(action_ref: String) -> Self {
        Self {
            action_ref,
            configuration: None,
            running: false,
            progress_lines: Vec::new(),
            outcome: None,
            cancelled: false,
            showing_details: false,
            summary_scroll: 0,
        }
    }

    pub fn selected_action(&self) -> &str {
        &self.action_ref
    }

    pub fn begin_configuration(&mut self, declarations: Vec<RunInputDeclarationResponse>) {
        self.configuration = Some(RunConfiguration::new(
            vec![self.action_ref.clone()],
            declarations,
        ));
    }

    pub fn begin_configuration_with_marker(
        &mut self,
        declarations: Vec<RunInputDeclarationResponse>,
        marker: &Marker,
    ) {
        self.configuration = Some(
            RunConfiguration::new(vec![self.action_ref.clone()], declarations).with_marker(marker),
        );
    }

    pub fn handle_configuration_key(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> ConfigurationAction {
        self.configuration
            .as_mut()
            .map(|configuration| configuration.handle_key(key))
            .unwrap_or(ConfigurationAction::Cancel)
    }

    pub fn take_configuration(&mut self) -> Option<RunConfigurationValues> {
        self.configuration
            .take()
            .and_then(|configuration| configuration.values())
    }

    pub fn configuration_footer(&self) -> Option<&'static str> {
        self.configuration.as_ref().map(RunConfiguration::footer)
    }

    pub fn configuration_is_editing(&self) -> bool {
        self.configuration
            .as_ref()
            .map(RunConfiguration::is_editing)
            .unwrap_or(false)
    }

    pub fn configuration_error(&self) -> Option<&str> {
        self.configuration
            .as_ref()
            .and_then(RunConfiguration::error)
    }

    pub fn report_configuration_error(&mut self, error: String) {
        if let Some(configuration) = self.configuration.as_mut() {
            configuration.report_error(error);
        }
    }

    pub fn start_run(&mut self) {
        self.configuration = None;
        self.outcome = None;
        self.cancelled = false;
        self.showing_details = false;
        self.summary_scroll = 0;
        self.progress_lines.clear();
        self.running = true;
    }

    pub fn record_progress(&mut self, line: String) {
        self.progress_lines
            .extend(line.lines().map(ToOwned::to_owned));
    }

    pub fn record_outcome(&mut self, outcome: ExecuteActionResponse) {
        self.outcome = Some(outcome);
        self.cancelled = false;
        self.running = false;
        self.showing_details = false;
        self.summary_scroll = 0;
    }

    pub fn record_cancelled(&mut self) {
        self.outcome = Some(ExecuteActionResponse::note("Cancelled".to_owned()));
        self.cancelled = true;
        self.running = false;
        self.showing_details = false;
        self.summary_scroll = 0;
    }

    pub fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn is_showing_details(&self) -> bool {
        self.showing_details
    }

    pub fn open_details(&mut self) {
        if self.outcome.is_some() {
            self.showing_details = true;
        }
    }

    pub fn close_details(&mut self) {
        self.showing_details = false;
    }

    pub fn scroll_summary_up(&mut self) {
        self.summary_scroll = self.summary_scroll.saturating_sub(1);
    }

    pub fn scroll_summary_down(&mut self) {
        self.summary_scroll = self.summary_scroll.saturating_add(1);
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        let title = if self.showing_details {
            "Action Details"
        } else if self.outcome.is_some() && !self.running {
            "Action Result"
        } else {
            Self::TITLE
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .padding(Padding::horizontal(1))
            .title(Span::styled(
                format!("{title}: {}", self.action_ref),
                Theme::title_style(),
            ));
        let content_area = block.inner(area);
        frame.render_widget(block, area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(content_area);
        if self.showing_details {
            frame.render_widget(
                Paragraph::new(self.details_text()).scroll((self.summary_scroll, 0)),
                chunks[0],
            );
        } else if let Some(configuration) = &self.configuration {
            configuration.render(frame, chunks[0]);
        } else if self.running {
            frame.render_widget(
                Paragraph::new(format!(
                    "Action is running...\n{}",
                    self.progress_lines.join("\n")
                )),
                chunks[0],
            );
        } else if let Some(outcome) = &self.outcome {
            let status = if self.cancelled {
                "CANCELLED"
            } else if outcome.exit_code() == 0 {
                "SUCCESS"
            } else {
                "FAILED"
            };
            frame.render_widget(
                Paragraph::new(format!(
                    "Status: {status}\nExit code: {}\n{}{}",
                    outcome.exit_code(),
                    outcome.stdout(),
                    outcome.stderr()
                ))
                .scroll((self.summary_scroll, 0)),
                chunks[0],
            );
        } else {
            frame.render_widget(Paragraph::new(""), chunks[0]);
        }
        frame.render_widget(
            Paragraph::new(self.footer()).style(Theme::muted_style()),
            chunks[1],
        );
    }

    fn details_text(&self) -> String {
        self.outcome
            .as_ref()
            .map(|outcome| {
                format!(
                    "stdout:\n{}\nstderr:\n{}",
                    outcome.stdout(),
                    outcome.stderr()
                )
            })
            .unwrap_or_else(|| "No action details available".to_owned())
    }

    pub fn footer(&self) -> &'static str {
        if self.showing_details {
            "Up/Down: Scroll | Esc/Bksp: Result"
        } else if self.outcome.is_some() && !self.running {
            "d: Details | Up/Down: Scroll | Esc/Bksp: Back | q: Quit"
        } else {
            self.configuration_footer().unwrap_or(if self.running {
                Self::RUNNING_FOOTER
            } else {
                Self::EMPTY_FOOTER
            })
        }
    }
}
