#[cfg(test)]
mod tests {
    use ephact::presentation::components::{box_component::BoxComponent, terminal::Terminal};

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
    fn centered_component_centers_box_and_content_in_terminal() {
        let rendered = BoxComponent::centered(|| "Title".to_owned(), &FakeTerminal).render();
        let lines = rendered.lines().collect::<Vec<_>>();

        assert!(lines.iter().all(|line| line.len() == 30));
        assert_eq!(lines[0], "    +-------------------+     ");
        assert!(lines.contains(&"    |       Title       |     "));
    }
}
