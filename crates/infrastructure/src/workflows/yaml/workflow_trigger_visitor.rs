use std::{collections::HashMap, fmt};

use serde::de::{Error, MapAccess, SeqAccess, Visitor};

use super::{TriggerFilterYaml, WorkflowTriggerYaml};

/// Reads the `on:` entry of a workflow, which may be a single event name, a
/// sequence of event names, or a mapping of event names to their filters.
pub struct WorkflowTriggerVisitor;

impl<'de> Visitor<'de> for WorkflowTriggerVisitor {
    type Value = WorkflowTriggerYaml;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string, sequence of strings, or mapping of event configs")
    }

    fn visit_str<E: Error>(self, event: &str) -> Result<WorkflowTriggerYaml, E> {
        Ok(WorkflowTriggerYaml::Single(event.to_owned()))
    }

    fn visit_string<E: Error>(self, event: String) -> Result<WorkflowTriggerYaml, E> {
        Ok(WorkflowTriggerYaml::Single(event))
    }

    fn visit_seq<A: SeqAccess<'de>>(
        self,
        mut sequence: A,
    ) -> Result<WorkflowTriggerYaml, A::Error> {
        let mut events = Vec::new();
        while let Some(event) = sequence.next_element::<String>()? {
            events.push(event);
        }
        Ok(WorkflowTriggerYaml::Multiple(events))
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<WorkflowTriggerYaml, M::Error> {
        let mut events = HashMap::new();
        while let Some((event, filter)) = map.next_entry::<String, Option<TriggerFilterYaml>>()? {
            events.insert(event, filter);
        }
        Ok(WorkflowTriggerYaml::WithTypes(events))
    }
}
