use std::error::Error;

use ephact::{
    application::ports::outbound::SettingsStorePort,
    infrastructure::{CargoProjectBrandingStore, Container, logging::stderr_filter::StderrFilter},
    presentation::{
        cli::run_progress_handler::RunProgressHandler, composition_root::CompositionRoot,
    },
};

pub struct AppRunner {
    _private: (),
}

impl AppRunner {
    pub fn run_application() {
        let stderr_filter = StderrFilter::install();
        let result = Self::run_application_impl();
        stderr_filter.restore();
        Self::finish(result, &ProcessExit);
    }

    fn run_application_impl() -> Result<(), Box<dyn Error>> {
        let verbose = std::env::args_os()
            .any(|arg| ephact::presentation::cli::RunArgs::is_verbose_flag(&arg));
        let (progress_reporter, progress_stream) = RunProgressHandler::with_tui_stream(verbose);
        let branding_store = CargoProjectBrandingStore::from_metadata(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            ephact::PROJECT_EMBLEM,
        );
        let container = Container::build_with_branding(
            Some(Box::new(progress_reporter)),
            Box::new(branding_store),
        );
        let settings_store = super::ConfigFactory::create_settings_store()?;
        let settings = settings_store.read_settings()?;
        let app = CompositionRoot::compose_with_tui_progress_and_settings(
            container,
            progress_stream,
            settings,
            settings_store,
        );
        app.run(std::env::args_os())
    }

    fn finish<E: ExitStrategy>(result: Result<(), Box<dyn Error>>, exit_strategy: &E) {
        match result {
            Ok(()) => exit_strategy.exit(0),
            Err(error) => {
                eprintln!("Error: {error}");
                exit_strategy.exit(1);
            }
        }
    }
}

trait ExitStrategy {
    fn exit(&self, code: i32);
}

struct ProcessExit;

impl ExitStrategy for ProcessExit {
    fn exit(&self, code: i32) {
        std::process::exit(code);
    }
}
