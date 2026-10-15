use super::{component::Component, terminal::Terminal};
use crate::components::text_alignment::TextAlignment;

pub struct BoxComponent<'a, C> {
    component: C,
    terminal: &'a dyn Terminal,
    alignment: TextAlignment,
    title: Option<String>,
}

impl<'a, C> BoxComponent<'a, C>
where
    C: Component,
{
    pub fn new(component: C, terminal: &'a dyn Terminal) -> Self {
        Self {
            component,
            terminal,
            alignment: TextAlignment::Start,
            title: None,
        }
    }

    pub fn centered(component: C, terminal: &'a dyn Terminal) -> Self {
        Self {
            alignment: TextAlignment::Center,
            ..Self::new(component, terminal)
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn render(&self) -> String {
        let (terminal_width, _) = self.terminal.dimensions();
        let box_width = terminal_width.saturating_mul(70) / 100;
        let width = box_width.saturating_sub(4);
        let content_lines: Vec<String> = self
            .component
            .render()
            .lines()
            .map(|line| line.chars().take(width).collect::<String>())
            .collect();
        let (top, bottom, left_edge, right_edge) = match self.title.as_deref() {
            Some(title) => {
                let title: String = title.chars().take(box_width.saturating_sub(2)).collect();
                let fill = "─".repeat(box_width.saturating_sub(title.chars().count() + 2));
                (
                    format!("┌{title}{fill}┐"),
                    format!("└{}┘", "─".repeat(box_width.saturating_sub(2))),
                    "│",
                    "│",
                )
            }
            None => (
                format!("+{}+", "-".repeat(width + 2)),
                format!("+{}+", "-".repeat(width + 2)),
                "|",
                "|",
            ),
        };
        let mut body = Vec::with_capacity(content_lines.len() + 6);
        body.extend(std::iter::repeat_n(String::new(), 3));
        body.extend(content_lines);
        body.extend(std::iter::repeat_n(String::new(), 3));

        let box_offset = TextAlignment::Center.offset(terminal_width, box_width);
        let right_padding = terminal_width.saturating_sub(box_offset + box_width);
        let left_padding = " ".repeat(box_offset);
        let right_padding = " ".repeat(right_padding);
        let mut lines = Vec::with_capacity(body.len() + 2);
        lines.push(format!("{left_padding}{top}{right_padding}"));
        lines.extend(body.into_iter().map(|line| {
            let content = self.alignment.line(&line, width);
            format!("{left_padding}{left_edge} {content} {right_edge}{right_padding}")
        }));
        lines.push(format!("{left_padding}{bottom}{right_padding}"));

        lines.join("\n") + "\n"
    }
}
