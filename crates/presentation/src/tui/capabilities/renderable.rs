use ratatui::{Frame, layout::Rect};

/// Common rendering contract for TUI components.
///
/// Components choose the context they need while sharing the same frame and area
/// arguments. Stateless components can use `()` as their context.
pub trait Renderable {
    type Context: ?Sized;

    fn render(&self, frame: &mut Frame<'_>, area: Rect, context: &Self::Context);
}
