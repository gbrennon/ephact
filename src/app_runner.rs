use std::sync::Arc;

use ephact::{
    application::ports::outbound::SettingsStorePort,
    infrastructure::{CargoProjectBrandingStore, Container, TomlSettingsStore},
    presentation::{
        cli::run_progress_handler::RunProgressHandler, composition_root::CompositionRoot,
    },
};

pub(super) fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let verbose =
        std::env::args_os().any(|arg| ephact::presentation::cli::RunArgs::is_verbose_flag(&arg));
    let (progress_reporter, progress_stream) = RunProgressHandler::with_tui_stream(verbose);
    let branding_store = CargoProjectBrandingStore::from_metadata(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
        ephact::PROJECT_EMBLEM,
    );
    let container =
        Container::build_with_branding(Some(Box::new(progress_reporter)), Box::new(branding_store));
    let settings_store = Arc::new(TomlSettingsStore::from_environment()?);
    let settings = settings_store.read_settings()?;
    let app = CompositionRoot::compose_with_tui_progress_and_settings(
        container,
        progress_stream,
        settings,
        settings_store,
    );
    app.run(std::env::args_os())
}
