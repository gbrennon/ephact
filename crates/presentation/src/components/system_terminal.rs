use std::io::{self, IsTerminal, Write};

use super::terminal::Terminal;

pub struct SystemTerminal;

impl SystemTerminal {
    fn normalize_dimensions(width: u16, height: u16) -> (usize, usize) {
        if width == 0 || height == 0 {
            (80, 24)
        } else {
            (usize::from(width), usize::from(height))
        }
    }
}

impl Terminal for SystemTerminal {
    fn dimensions(&self) -> (usize, usize) {
        crossterm::terminal::size()
            .map(|(width, height)| Self::normalize_dimensions(width, height))
            .unwrap_or((80, 24))
    }

    fn write_text(&self, text: &str) -> io::Result<()> {
        print!("{text}");
        io::stdout().flush()
    }

    fn read_line(&self) -> io::Result<String> {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok(line)
    }

    fn is_interactive(&self) -> bool {
        io::stdin().is_terminal()
    }
}

#[cfg(test)]
mod tests {
    use super::SystemTerminal;

    #[test]
    fn zero_terminal_dimensions_use_cli_fallback() {
        assert_eq!(SystemTerminal::normalize_dimensions(0, 0), (80, 24));
    }

    #[test]
    fn valid_terminal_dimensions_are_preserved() {
        assert_eq!(SystemTerminal::normalize_dimensions(120, 40), (120, 40));
    }
}
