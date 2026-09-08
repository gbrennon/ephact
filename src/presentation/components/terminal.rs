pub trait Terminal {
    fn dimensions(&self) -> (usize, usize);
}

pub struct SystemTerminal;

impl Terminal for SystemTerminal {
    fn dimensions(&self) -> (usize, usize) {
        crossterm::terminal::size()
            .map(|(width, height)| (usize::from(width), usize::from(height)))
            .unwrap_or((80, 24))
    }
}
