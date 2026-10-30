use std::error::Error;

use ephact::infrastructure::logging::stderr_filter::StderrFilter;

pub fn run() {
    let stderr_filter = StderrFilter::install();
    let result = ephact::app_runner::run_application();
    stderr_filter.restore();
    finish(result);
}

fn finish(result: Result<(), Box<dyn Error>>) {
    match result {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}
