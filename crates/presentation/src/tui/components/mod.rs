use ratatui::{Frame, layout::Rect};

/// Common rendering contract for TUI components.
///
/// Components choose the context they need while sharing the same frame and area
/// arguments. Stateless components can use `()` as their context.
pub trait Renderable {
    type Context: ?Sized;

    fn render(&self, frame: &mut Frame<'_>, area: Rect, context: &Self::Context);
}

pub mod color_support;
pub mod emblem;
pub mod run_configuration;
mod run_configuration_input;
pub mod run_details_view;
pub mod screen_frame;
pub mod splash_quotes;

pub use color_support::ColorSupport;
pub use emblem::Emblem;
pub use run_configuration::{ConfigurationAction, RunConfiguration, RunConfigurationValues};
pub use run_details_view::RunDetailsView;
pub use screen_frame::ScreenFrame;
pub use splash_quotes::SplashQuotes;
