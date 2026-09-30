use std::collections::HashMap;

use super::{TriggerFilter, TriggerInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerKind {
    Push,
    PullRequest,
    Manual,
    Schedule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowTrigger {
    Push(Option<TriggerFilter>),
    PullRequest(Option<TriggerFilter>),
    Manual {
        inputs: HashMap<String, TriggerInput>,
    },
    Schedule {
        expressions: Vec<String>,
    },
}

impl WorkflowTrigger {
    pub fn kind(&self) -> TriggerKind {
        match self {
            Self::Push(_) => TriggerKind::Push,
            Self::PullRequest(_) => TriggerKind::PullRequest,
            Self::Manual { .. } => TriggerKind::Manual,
            Self::Schedule { .. } => TriggerKind::Schedule,
        }
    }

    pub fn filter(&self) -> Option<&TriggerFilter> {
        match self {
            Self::Push(filter) | Self::PullRequest(filter) => filter.as_ref(),
            Self::Manual { .. } | Self::Schedule { .. } => None,
        }
    }

    pub fn inputs(&self) -> Option<&HashMap<String, TriggerInput>> {
        match self {
            Self::Manual { inputs } => Some(inputs),
            Self::Push(_) | Self::PullRequest(_) | Self::Schedule { .. } => None,
        }
    }

    pub fn expressions(&self) -> &[String] {
        match self {
            Self::Schedule { expressions } => expressions,
            Self::Push(_) | Self::PullRequest(_) | Self::Manual { .. } => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_domain_trigger_kinds() {
        assert_eq!(WorkflowTrigger::Push(None).kind(), TriggerKind::Push);
        assert_eq!(
            WorkflowTrigger::PullRequest(None).kind(),
            TriggerKind::PullRequest
        );
        assert_eq!(
            WorkflowTrigger::Manual {
                inputs: HashMap::new()
            }
            .kind(),
            TriggerKind::Manual
        );
        assert_eq!(
            WorkflowTrigger::Schedule {
                expressions: vec!["0 0 * * *".into()]
            }
            .kind(),
            TriggerKind::Schedule
        );
    }

    #[test]
    fn exposes_manual_inputs_without_vendor_event_names() {
        let inputs = HashMap::from([(
            "environment".into(),
            TriggerInput::new(None, true, None, None, Vec::new()),
        )]);
        let trigger = WorkflowTrigger::Manual {
            inputs: inputs.clone(),
        };

        assert_eq!(trigger.inputs(), Some(&inputs));
    }

    #[test]
    fn filter_is_returned_only_for_push_and_pull_request() {
        let filter = TriggerFilter::default();

        assert_eq!(
            WorkflowTrigger::Push(Some(filter.clone())).filter(),
            Some(&filter)
        );
        assert_eq!(
            WorkflowTrigger::PullRequest(Some(filter.clone())).filter(),
            Some(&filter)
        );
        assert!(
            WorkflowTrigger::Manual {
                inputs: HashMap::new()
            }
            .filter()
            .is_none()
        );
    }

    #[test]
    fn inputs_are_absent_for_non_manual_triggers() {
        assert!(WorkflowTrigger::Push(None).inputs().is_none());
    }

    #[test]
    fn expressions_are_returned_only_for_schedule_triggers() {
        let schedule = WorkflowTrigger::Schedule {
            expressions: vec!["0 0 * * *".into()],
        };

        assert_eq!(schedule.expressions(), &["0 0 * * *".to_string()]);
        assert!(WorkflowTrigger::Push(None).expressions().is_empty());
    }
}
