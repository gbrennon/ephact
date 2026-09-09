use super::{component::Component, terminal::Terminal};

pub struct BoxComponent<'a, C> {
    component: C,
    terminal: &'a dyn Terminal,
}

impl<'a, C> BoxComponent<'a, C>
where
    C: Component,
{
    pub fn new(component: C, terminal: &'a dyn Terminal) -> Self {
        Self {
            component,
            terminal,
        }
    }

    pub fn render(&self) -> String {
        let (terminal_width, _) = self.terminal.dimensions();
        let box_width = terminal_width.saturating_mul(70) / 100;
        let width = box_width.saturating_sub(4);
        let content_lines: Vec<String> = self
            .component
            .render()
            .lines()
            .map(|line| line.chars().take(width).collect())
            .collect();
        let border = format!("+{}+", "-".repeat(width + 2));
        let mut body = Vec::with_capacity(content_lines.len() + 6);
        body.extend(std::iter::repeat_n(String::new(), 3));
        body.extend(content_lines);
        body.extend(std::iter::repeat_n(String::new(), 3));

        let mut lines = Vec::with_capacity(body.len() + 2);
        lines.push(border.clone());
        lines.extend(
            body.into_iter()
                .map(|line| format!("| {line:<width$} |", width = width)),
        );
        lines.push(border);

        lines.join("\n") + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::super::{content::ContentComponent, terminal::Terminal};
    use super::BoxComponent;

    struct FakeTerminal;

    impl Terminal for FakeTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (30, 12)
        }

        fn write_text(&self, _text: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(String::new())
        }
    }

    #[test]
    fn renders_injected_component_at_shared_terminal_width_with_padding() {
        let terminal = FakeTerminal;
        let component = ContentComponent::new("Title".to_string(), "injected content".to_string());

        let rendered = BoxComponent::new(component, &terminal).render();

        assert_eq!(rendered.lines().count(), 11);
        assert!(rendered.lines().all(|line| line.chars().count() == 21));
    }

    #[test]
    fn renders_a_closure_as_an_injected_component() {
        let terminal = FakeTerminal;
        let rendered = BoxComponent::new(|| "closure content".to_string(), &terminal).render();

        assert!(rendered.contains("closure content"));
    }
}
