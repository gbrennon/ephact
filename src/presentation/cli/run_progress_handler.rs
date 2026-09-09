use std::io::Write;

use crate::application::ports::outbound::DomainEventHandler;
use crate::domain::events::{
    DomainEvent, JobStartedPayload,
    StepFinishedPayload, StepOutputPayload,
};
/// Presentation handler that renders workflow run progress to the terminal.
///
/// In non-verbose mode, steps are announced as they begin and their outcome is
/// reported on completion, while workflow and job headers, live step output,
/// and failure diagnostics remain hidden.
///
/// In verbose mode, workflow and job lifecycle events are announced, step output
/// is relayed in real time, and failure diagnostics are appended on step completion.
///
/// Everything is written to standard error so that step output relayed to
/// standard output stays clean.
pub struct RunProgressHandler {
    verbose: bool,
}

impl RunProgressHandler {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    fn write_line(line: &str) {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "{line}");
        let _ = stderr.flush();
    }

    fn status(success: bool) -> &'static str {
        if success { "succeeded" } else { "failed" }
    }

    fn job_label(payload: &JobStartedPayload) -> String {
        match payload.job_name() {
            Some(name) => format!("{} ({})", payload.job_id(), name),
            None => payload.job_id().to_string(),
        }
    }

    fn relay_output(payload: &StepOutputPayload) {
        let mut stderr = std::io::stderr().lock();
        let _ = write!(stderr, "      | ");
        let _ = stderr.write_all(payload.text().as_bytes());
        let _ = stderr.flush();
    }

    fn step_status(exit_code: Option<i64>) -> String {
        match exit_code {
            Some(0) => "ok".to_string(),
            Some(code) => format!("failed (exit code: {code})"),
            None => "error".to_string(),
        }
    }

    fn step_outcome(&self, payload: &StepFinishedPayload) -> String {
        let outcome = Self::step_status(payload.exit_code());
        let mut output = format!("    Step '{}': {outcome}", payload.step_name());
        if self.verbose && payload.exit_code() != Some(0) {
            Self::append_failure_output(&mut output, "stdout", payload.stdout());
            Self::append_failure_output(&mut output, "stderr", payload.stderr());
        }
        output
    }

    fn append_failure_output(output: &mut String, label: &str, text: &str) {
        if text.is_empty() {
            return;
        }
        for line in text.lines() {
            output.push_str(&format!("\n      {label}: {line}"));
        }
    }

    /// Renders the terminal line for an event, or `None` when the event is
    /// not shown in the current verbosity mode.
    fn render(&self, event: &DomainEvent) -> Option<String> {
        match event {
            DomainEvent::WorkflowStarted(payload) if self.verbose => {
                Some(format!("Workflow '{}'", payload.workflow_name()))
            }
            DomainEvent::JobStarted(payload) if self.verbose => {
                Some(format!("  Job '{}'", Self::job_label(payload)))
            }
            DomainEvent::JobFinished(payload) if self.verbose => Some(format!(
                "  Job '{}': {}",
                payload.job_id(),
                Self::status(payload.success())
            )),
            DomainEvent::StepStarted(payload) => {
                Some(format!("    Step '{}': running...", payload.step_name()))
            }
            DomainEvent::StepFinished(payload) => Some(self.step_outcome(payload)),
            _ => None,
        }
    }

    fn renders_output(&self) -> bool {
        self.verbose
    }
}

impl DomainEventHandler for RunProgressHandler {
    fn handle(&self, event: &DomainEvent) {
        match event {
            DomainEvent::StepOutput(payload) if self.renders_output() => {
                Self::relay_output(payload)
            }
            other => {
                if let Some(line) = self.render(other) {
                    Self::write_line(&line);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::events::{
        JobFinishedPayload, JobStartedPayload, OutputStream, StepFinishedPayload,
        StepOutputPayload, StepStartedPayload, WorkflowStartedPayload,
    };

    fn step_started() -> DomainEvent {
        DomainEvent::StepStarted(StepStartedPayload::new(
            "Build".into(),
            "build".into(),
            "compile".into(),
        ))
    }

    fn step_output(stream: OutputStream) -> DomainEvent {
        DomainEvent::StepOutput(StepOutputPayload::new(
            "compile".into(),
            stream,
            "Compiling ephact\n".into(),
        ))
    }

    fn step_finished(exit_code: Option<i64>) -> DomainEvent {
        DomainEvent::StepFinished(StepFinishedPayload::new(
            "Build".into(),
            "build".into(),
            "compile".into(),
            exit_code == Some(0),
            exit_code,
            String::new(),
            String::new(),
        ))
    }

    #[test]
    fn quiet_mode_announces_running_steps() {
        let handler = RunProgressHandler::new(false);
        assert_eq!(
            handler.render(&step_started()).as_deref(),
            Some("    Step 'compile': running...")
        );
    }

    #[test]
    fn quiet_mode_reports_step_status() {
        let handler = RunProgressHandler::new(false);
        assert_eq!(
            handler.render(&step_finished(Some(0))).as_deref(),
            Some("    Step 'compile': ok")
        );
        assert!(!handler.renders_output());
    }

    #[test]
    fn quiet_mode_hides_workflow_and_job_headers() {
        let handler = RunProgressHandler::new(false);
        assert!(
            handler
                .render(&DomainEvent::WorkflowStarted(WorkflowStartedPayload::new(
                    "Build".into(),
                )))
                .is_none()
        );
        assert!(
            handler
                .render(&DomainEvent::JobStarted(JobStartedPayload::new(
                    "Build".into(),
                    "build".into(),
                    Some("Build".into()),
                )))
                .is_none()
        );
        assert!(
            handler
                .render(&DomainEvent::JobFinished(JobFinishedPayload::new(
                    "Build".into(),
                    "build".into(),
                    Some("Build".into()),
                    true,
                )))
                .is_none()
        );
    }

    #[test]
    fn quiet_mode_hides_failed_step_output() {
        let handler = RunProgressHandler::new(false);
        let event = DomainEvent::StepFinished(StepFinishedPayload::new(
            "Build".into(),
            "build".into(),
            "clippy".into(),
            false,
            Some(101),
            "stdout text".into(),
            "clippy failed".into(),
        ));

        let rendered = handler.render(&event);
        assert_eq!(
            rendered.as_deref(),
            Some("    Step 'clippy': failed (exit code: 101)")
        );
        let rendered_text = rendered.unwrap();
        assert!(!rendered_text.contains("stdout"));
        assert!(!rendered_text.contains("clippy failed"));
    }

    #[test]
    fn verbose_mode_reports_failed_step_output() {
        let handler = RunProgressHandler::new(true);
        let event = DomainEvent::StepFinished(StepFinishedPayload::new(
            "Build".into(),
            "build".into(),
            "clippy".into(),
            false,
            Some(101),
            "stdout text".into(),
            "clippy failed".into(),
        ));

        let rendered = handler.render(&event).unwrap();
        assert!(rendered.contains("Step 'clippy': failed (exit code: 101)"));
        assert!(rendered.contains("stderr: clippy failed"));
    }
    #[test]
    fn verbose_mode_announces_running_steps() {
        let handler = RunProgressHandler::new(true);
        assert_eq!(
            handler.render(&step_started()).as_deref(),
            Some("    Step 'compile': running...")
        );
        assert!(handler.renders_output());
    }

    #[test]
    fn verbose_mode_relays_step_output_and_quiet_mode_does_not() {
        let verbose = RunProgressHandler::new(true);
        let quiet = RunProgressHandler::new(false);
        let event = step_output(OutputStream::StandardOutput);
        assert!(verbose.render(&event).is_none() && verbose.renders_output());
        assert!(!quiet.renders_output());
    }

    #[test]
    fn verbose_mode_still_reports_the_step_status() {
        let handler = RunProgressHandler::new(true);
        assert_eq!(
            handler.render(&step_finished(Some(0))).as_deref(),
            Some("    Step 'compile': ok")
        );
    }
}
