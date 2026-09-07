/// Terminal stream a step's output was written to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStream {
    StandardOutput,
    StandardError,
}
