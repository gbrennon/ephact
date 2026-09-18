use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub struct HomeScreen;

impl HomeScreen {
    pub fn render(frame: &mut Frame<'_>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(5)])
            .split(frame.area());

        let title = Paragraph::new("EPHACT")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL).title("ephact"));
        frame.render_widget(title, chunks[0]);

        let menu = List::new([
            ListItem::new(Line::from(Span::raw("Run workflow"))),
            ListItem::new(Line::from(Span::raw("List workflows"))),
            ListItem::new(Line::from(Span::raw("List actions"))),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("What would you like to do?"),
        );
        frame.render_widget(menu, chunks[1]);
    }
}
