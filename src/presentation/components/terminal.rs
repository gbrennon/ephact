use std::io::{self, IsTerminal, Write};

pub trait Terminal {
    fn dimensions(&self) -> (usize, usize);

    fn write_text(&self, text: &str) -> io::Result<()>;

    fn read_line(&self) -> io::Result<String>;

    fn is_interactive(&self) -> bool {
        true
    }
}

pub struct SystemTerminal;

impl Terminal for SystemTerminal {
    fn dimensions(&self) -> (usize, usize) {
        crossterm::terminal::size()
            .map(|(width, height)| (usize::from(width), usize::from(height)))
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
