use std::io;

pub trait Terminal {
    fn dimensions(&self) -> (usize, usize);

    fn write_text(&self, text: &str) -> io::Result<()>;

    fn read_line(&self) -> io::Result<String>;

    fn is_interactive(&self) -> bool {
        true
    }
}

pub use super::system_terminal::SystemTerminal;
