use std::io::Write;

use crate::{
    application::ports::outbound::DomainEventHandler,
    domain::events::{
        DomainEvent, JobStartedPayload, StepFinishedPayload, StepOutputPayload,
        WorkflowStartedPayload,
    },
};

/// Presentation handler that renders workflow run progress to the terminal.
///
/// Without verbose mode only the final status of each step is shown; with
/// verbose mode every step is announced while running and its output is
/// relayed in real time.
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
        match &payload.job_name {
            Some(name) => format!("{} ({})", payload.job_id, name),
            None => payload.job_id.clone(),
        }
    }

    fn relay_output(payload: &StepOutputPayload) {
        let mut stderr = std::io::stderr().lock();
        let _ = write!(stderr, "      | ");
        let _ = stderr.write_all(payload.text.as_bytes());
        let _ = stderr.flush();
    }

    fn step_outcome(payload: &StepFinishedPayload) -> String {
        let outcome = match payload.exit_code {
            Some(0) => "ok".to_string(),
            Some(code) => format!("failed (exit code: {code})"),
            None => "error".to_string(),
        };
        let mut output = format!("    Step '{}': {outcome}", payload.step_name);
        if payload.exit_code != Some(0) {
            Self::append_failure_output(&mut output, "stdout", &payload.stdout);
            Self::append_failure_output(&mut output, "stderr", &payload.stderr);
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
            DomainEvent::WorkflowStarted(WorkflowStartedPayload { workflow_name }) => {
                Some(format!("Workflow '{workflow_name}'"))
            }
            DomainEvent::JobStarted(payload) => {
                Some(format!("  Job '{}'", Self::job_label(payload)))
            }
            DomainEvent::JobFinished(payload) => Some(format!(
                "  Job '{}': {}",
                payload.job_id,
                Self::status(payload.success)
            )),
            DomainEvent::StepStarted(payload) if self.verbose => {
                Some(format!("    Step '{}': running...", payload.step_name))
            }
            DomainEvent::StepFinished(payload) => Some(Self::step_outcome(payload)),
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
        OutputStream, StepFinishedPayload, StepOutputPayload, StepStartedPayload,
    };

    fn step_started() -> DomainEvent {
        DomainEvent::StepStarted(StepStartedPayload {
            workflow_name: "Build".into(),
            job_id: "build".into(),
            step_name: "compile".into(),
        })
    }

    fn step_output(stream: OutputStream) -> DomainEvent {
        DomainEvent::StepOutput(StepOutputPayload {
            step_name: "compile".into(),
            stream,
            text: "Compiling ephact\n".into(),
        })
    }

    fn step_finished(exit_code: Option<i64>) -> DomainEvent {
        DomainEvent::StepFinished(StepFinishedPayload {
            workflow_name: "Build".into(),
            job_id: "build".into(),
            step_name: "compile".into(),
            success: exit_code == Some(0),
            exit_code,
            stdout: String::new(),
            stderr: String::new(),
        })
    }

    #[test]
    fn quiet_mode_shows_only_the_step_status() {
        let handler = RunProgressHandler::new(false);
        assert!(handler.render(&step_started()).is_none());
        assert!(!handler.renders_output());
        assert_eq!(
            handler.render(&step_finished(Some(0))).as_deref(),
            Some("    Step 'compile': ok")
        );
    }

    #[test]
    fn quiet_mode_still_shows_workflow_and_job_headers() {
        let handler = RunProgressHandler::new(false);
        assert_eq!(
            handler
                .render(&DomainEvent::WorkflowStarted(WorkflowStartedPayload {
                    workflow_name: "Build".into(),
                }))
                .as_deref(),
            Some("Workflow 'Build'")
        );
        assert_eq!(
            handler
                .render(&DomainEvent::JobStarted(JobStartedPayload {
                    workflow_name: "Build".into(),
                    job_id: "build".into(),
                    job_name: Some("Build".into()),
                }))
                .as_deref(),
            Some("  Job 'build (Build)'")
        );
    }

    #[test]
    fn quiet_mode_reports_a_failed_step_with_its_exit_code() {
        let handler = RunProgressHandler::new(false);
        assert_eq!(
            handler.render(&step_finished(Some(2))).as_deref(),
            Some("    Step 'compile': failed (exit code: 2)")
        );
    }

    #[test]
    fn quiet_mode_reports_failed_step_output() {
        let handler = RunProgressHandler::new(false);
        let event = DomainEvent::StepFinished(StepFinishedPayload {
            workflow_name: "Build".into(),
            job_id: "build".into(),
            step_name: "clippy".into(),
            success: false,
            exit_code: Some(101),
            stdout: String::new(),
            stderr: "clippy failed".into(),
        });

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
