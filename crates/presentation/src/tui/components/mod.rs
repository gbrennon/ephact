pub mod color_support;
pub mod emblem;
pub mod run_configuration;
mod run_configuration_input;
mod run_configuration_values;
pub mod run_details_view;
pub mod screen_frame;

pub use color_support::ColorSupport;
pub use emblem::{Emblem, PROJECT_EMBLEM};
pub use run_configuration::{ConfigurationAction, RunConfiguration};
pub use run_configuration_values::RunConfigurationValues;
pub use run_details_view::RunDetailsView;
pub use screen_frame::ScreenFrame;

pub use crate::tui::capabilities::Renderable;
