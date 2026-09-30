//! Woodpecker-exclusive implementations.
//!
//! Everything in this module is specific to the Woodpecker CI pipeline format
//! (`.woodpecker/`), whose schema differs from the GitHub/Forgejo Actions layout
//! the rest of the crate speaks. Shared concerns (directory detection, listing)
//! stay in the parent module; only code that has no meaning outside Woodpecker
//! lives here.
//!
//! The Woodpecker schema uses top-level `steps:` (each with its own `image` and
//! `commands`) gated by `when:` conditions, instead of the Actions `on:`/`jobs:`
//! shape. [`WoodpeckerPipelineYaml`] parses that schema and maps it onto the
//! shared domain [`crate::domain::aggregates::Workflow`] aggregate: each
//! Woodpecker step becomes a single-step domain job carrying the step image as
//! its container and chained to its predecessor via `needs` to preserve order.

pub mod woodpecker_pipeline_yaml;
pub mod woodpecker_step_yaml;
pub mod woodpecker_when_yaml;

pub use woodpecker_pipeline_yaml::WoodpeckerPipelineYaml;
pub use woodpecker_step_yaml::WoodpeckerStepYaml;
pub use woodpecker_when_yaml::WoodpeckerWhenYaml;
