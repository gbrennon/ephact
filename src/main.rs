mod app_runner;

use ephact::infrastructure::logging::stderr_filter::StderrFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stderr_filter = StderrFilter::install();
    let result = app_runner::run_application();
    stderr_filter.restore();

    match result {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}
