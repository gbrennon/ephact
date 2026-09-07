use ephact::infrastructure::logging::stderr_filter::StderrFilter;
use ephact::{
    infrastructure::Container,
    presentation::{
        banner, cli::run_progress_handler::RunProgressHandler, composition_root::CompositionRoot,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", banner::render());
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
    let app = CompositionRoot::compose(
        container.run_workflow_port,
        container.run_all_workflows_port,
        container.list_workflows_port,
        container.list_actions_port,
    );
    app.cli.run(std::env::args_os())
}
