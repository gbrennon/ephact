use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub const EMBLEM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/project_emblem.txt"
));

pub struct SplashScreen;

impl SplashScreen {
    pub fn render(frame: &mut Frame<'_>) {
        let area = Self::centered_area(frame.area(), 60, 70);
        let mut lines: Vec<Line<'_>> = EMBLEM.lines().map(Line::from).collect();
        lines.extend([Line::from(""), Line::from("EPHACT")]);
        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Welcome"))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(paragraph, area);
    }

    fn centered_area(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - height_percent) / 2),
                Constraint::Percentage(height_percent),
                Constraint::Percentage((100 - height_percent) / 2),
            ])
            .split(area);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - width_percent) / 2),
                Constraint::Percentage(width_percent),
                Constraint::Percentage((100 - width_percent) / 2),
            ])
            .split(vertical[1])[1]
    }
}
