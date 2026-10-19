use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    os::{fd::AsFd, unix::io::OwnedFd},
    thread::JoinHandle,
};

use rustix::{io::dup, pipe::pipe, stdio::dup2_stderr};

use super::StderrFilterConfig;

/// Duplicated stderr whose configured line prefixes are dropped before reaching
/// the terminal.
pub struct StderrFilter {
    filter_thread: Option<JoinHandle<()>>,
    restored_fd: Option<OwnedFd>,
}

impl StderrFilter {
    /// Installs a pipe in front of process stderr with the default suppression
    /// prefixes and spawns the draining thread.
    pub fn install() -> Self {
        Self::install_with_config(StderrFilterConfig::default())
    }

    /// Installs a pipe in front of process stderr with configured suppression
    /// prefixes and spawns the draining thread.
    pub fn install_with_config(config: StderrFilterConfig) -> Self {
        let stderr = io::stderr();
        let restored_fd = match dup(stderr.as_fd()) {
            Ok(fd) => fd,
            Err(_) => return Self::disabled(),
        };
        let (read_end, write_end) = match pipe() {
            Ok(fds) => fds,
            Err(_) => return Self::disabled(),
        };
        let thread_stderr = match dup(&restored_fd) {
            Ok(fd) => fd,
            Err(_) => return Self::disabled(),
        };
        if dup2_stderr(&write_end).is_err() {
            return Self::disabled();
        }
        let filter_thread = Some(Self::spawn_filter_thread(read_end, thread_stderr, config));
        drop(write_end);

        Self {
            filter_thread,
            restored_fd: Some(restored_fd),
        }
    }

    /// Restores the original stderr and waits for the draining thread.
    pub fn restore(self) {
        let Self {
            filter_thread,
            restored_fd,
        } = self;
        if let Some(restored_fd) = restored_fd.as_ref() {
            let _ = dup2_stderr(restored_fd);
        }
        if let Some(handle) = filter_thread {
            let _ = handle.join();
        }
    }

    fn disabled() -> Self {
        Self {
            filter_thread: None,
            restored_fd: None,
        }
    }

    fn line_should_be_suppressed(line: &str, config: &StderrFilterConfig) -> bool {
        config
            .suppressed_line_prefixes()
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }
    fn spawn_filter_thread(
        read_end: OwnedFd,
        real_stderr: OwnedFd,
        config: StderrFilterConfig,
    ) -> JoinHandle<()> {
        std::thread::spawn(move || {
            let reader = BufReader::new(File::from(read_end));
            let mut real = File::from(real_stderr);
            for line in reader.lines().map_while(Result::ok) {
                if StderrFilter::line_should_be_suppressed(&line, &config) {
                    continue;
                }
                let _ = writeln!(real, "{line}");
                let _ = real.flush();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crun_missing_bash_line_is_suppressed() {
        let config = StderrFilterConfig::default();

        assert!(StderrFilter::line_should_be_suppressed(
            "Error: crun: executable file `bash` not found in $PATH",
            &config,
        ));
    }

    #[test]
    fn unix_packet_error_line_is_suppressed() {
        let config = StderrFilterConfig::default();

        assert!(StderrFilter::line_should_be_suppressed(
            "Error: read unixpacket @/run/whatever: broken pipe",
            &config,
        ));
    }

    #[test]
    fn ordinary_lines_are_not_suppressed() {
        let config = StderrFilterConfig::default();

        assert!(!StderrFilter::line_should_be_suppressed(
            "Compiling ephact v0.1.0",
            &config,
        ));
        assert!(!StderrFilter::line_should_be_suppressed("", &config));
        assert!(!StderrFilter::line_should_be_suppressed(
            "Warning: crun: executable file `bash` not found",
            &config,
        ));
    }

    #[test]
    fn configured_prefixes_replace_default_prefixes() {
        let config = StderrFilterConfig::from_prefixes(vec!["Custom:".to_owned()]);

        assert!(StderrFilter::line_should_be_suppressed(
            "Custom: message",
            &config,
        ));
        assert!(!StderrFilter::line_should_be_suppressed(
            "Error: read unixpacket @/run/whatever: broken pipe",
            &config,
        ));
    }

    #[test]
    fn empty_configured_prefixes_do_not_suppress_lines() {
        let config = StderrFilterConfig::from_prefixes(vec![String::new()]);

        assert!(!StderrFilter::line_should_be_suppressed(
            "ordinary output",
            &config
        ));
    }

    #[test]
    fn filter_thread_does_not_close_the_original_stderr_descriptor() {
        let stderr = io::stderr();
        let original_stderr = dup(stderr.as_fd()).expect("stderr must be duplicable");
        let thread_stderr = dup(&original_stderr).expect("stderr must be duplicable");
        let (read_end, write_end) = pipe().expect("pipe must be creatable");
        let config = StderrFilterConfig::default();

        let handle = StderrFilter::spawn_filter_thread(read_end, thread_stderr, config);
        drop(write_end);
        handle.join().expect("filter thread must not panic");

        assert!(dup(&original_stderr).is_ok());
    }

    #[test]
    fn install_and_restore_leave_stderr_usable() {
        let filter = StderrFilter::install();

        eprintln!("message written through the installed filter");
        filter.restore();
        eprintln!("message written after restore");
    }
}
