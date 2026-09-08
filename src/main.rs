use ephact::infrastructure::logging::stderr_filter::StderrFilter;
use ephact::{
    infrastructure::Container,
    presentation::{
        cli::run_progress_handler::RunProgressHandler, composition_root::CompositionRoot,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stderr_filter = StderrFilter::install();
    let result = run_application();
    stderr_filter.restore();

    match result {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}

fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let verbose =
        std::env::args_os().any(|arg| ephact::presentation::cli::RunArgs::is_verbose_flag(&arg));
    let container = Container::build(Some(Box::new(RunProgressHandler::new(verbose))));
    let app = CompositionRoot::compose(container);
    app.cli.run(std::env::args_os())
}
